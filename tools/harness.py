#!/usr/bin/env python3
"""
Human-in-the-loop pipeline harness.

Goals:
- Parse pipeline.yaml and stage markdown front matter
- Resolve includes (rules/runner/profile) and library/artifact inputs
- Assemble a compiled prompt per stage into dist/
- Optionally capture model output and write artifacts

This harness intentionally does NOT call any LLM APIs. It is designed for copy/paste workflows.
"""
from __future__ import annotations

import argparse
import datetime as _dt
import os
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

try:
    import yaml  # type: ignore
except ModuleNotFoundError:  # pragma: no cover
    import yaml_lite as yaml  # type: ignore


ROOT = Path(__file__).resolve().parents[1]  # handbook/
DIST_DIR = ROOT / "dist"
ARTIFACTS_DIR = ROOT / "artifacts"

HELP_BANNER = (
    "Legacy reference material only.\n"
    "Rust-first is the supported direction. See docs/contracts/C-01-approved-repo-surface.md.\n"
    "Allowed changes: bug fixes, link corrections, and narrow wording fixes only until cutover."
)


FRONT_MATTER_RE = re.compile(r"^---\s*$", re.M)

# Optional scoped-block filtering for included markdown content.
#
# Syntax:
#   <!-- SCOPE: L2,L3 -->
#   ... content ...
#   <!-- END_SCOPE -->
#
# If a stage has a `work_level` (L0..L3), only blocks whose scope contains that
# level are included in the compiled prompt.
SCOPE_START_RE = re.compile(r"^\s*<!--\s*SCOPE:\s*([A-Za-z0-9_,\s]+)\s*-->\s*$")
SCOPE_END_RE = re.compile(r"^\s*<!--\s*END_SCOPE\s*-->\s*$")


def _read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="replace")


def _write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def filter_scoped_blocks(text: str, work_level: str | None) -> str:
    """Filter optional scoped blocks from markdown content."""
    if not text:
        return text
    if not work_level:
        return text

    wl = str(work_level).strip()
    if not wl:
        return text

    out_lines: List[str] = []
    include = True
    for line in text.splitlines():
        m = SCOPE_START_RE.match(line)
        if m:
            levels = {x.strip() for x in m.group(1).split(",") if x.strip()}
            include = ("ALL" in levels) or (wl in levels)
            continue
        if SCOPE_END_RE.match(line):
            include = True
            continue
        if include:
            out_lines.append(line)

    # Preserve a single trailing newline if the input had one.
    out = "\n".join(out_lines).rstrip() + "\n"
    return out


def parse_markdown_front_matter(md_path: Path) -> Tuple[Dict[str, Any], str]:
    """
    Returns (front_matter_dict, body_text).
    If no front matter exists, returns ({}, full_text).
    """
    text = _read_text(md_path)
    # Must start with '---' on first non-empty line.
    lines = text.splitlines()
    i = 0
    while i < len(lines) and lines[i].strip() == "":
        i += 1
    if i >= len(lines) or lines[i].strip() != "---":
        return {}, text

    # Find closing '---'
    j = i + 1
    while j < len(lines):
        if lines[j].strip() == "---":
            break
        j += 1
    if j >= len(lines):
        # malformed; treat as no front matter
        return {}, text

    fm_text = "\n".join(lines[i + 1 : j])
    body = "\n".join(lines[j + 1 :])  # may be empty
    try:
        fm = yaml.safe_load(fm_text) or {}
        if not isinstance(fm, dict):
            fm = {}
    except Exception:
        fm = {}
    return fm, body


def _substitute_vars(s: str, variables: Dict[str, Any]) -> str:
    def repl(m: re.Match[str]) -> str:
        key = m.group(1)
        v = variables.get(key)
        return str(v) if v is not None else m.group(0)

    return re.sub(r"\$\{([^}]+)\}", repl, s)


def _coerce_bool(v: Any) -> Optional[bool]:
    if isinstance(v, bool):
        return v
    if isinstance(v, str):
        if v.lower() in {"true", "yes", "1"}:
            return True
        if v.lower() in {"false", "no", "0"}:
            return False
    return None


def eval_activation(activation: Dict[str, Any], variables: Dict[str, Any]) -> bool:
    """
    Very small activation evaluator.

    Supported:
    activation:
      when:
        any:
          - variables.foo == true
          - variables.bar == "baz"
    """
    if not activation:
        return True
    when = activation.get("when") or {}
    any_list = when.get("any")
    all_list = when.get("all")

    def eval_expr(expr: str) -> bool:
        expr = expr.strip()
        # variables.<name> == <literal>
        m = re.match(r"variables\.([A-Za-z0-9_]+)\s*==\s*(.+)$", expr)
        if not m:
            return False
        name = m.group(1)
        raw_rhs = m.group(2).strip()
        lhs = variables.get(name)
        # rhs literal
        if raw_rhs.lower() in {"true", "false"}:
            rhs = raw_rhs.lower() == "true"
            lhsb = _coerce_bool(lhs)
            return lhsb == rhs
        # quoted string
        if (raw_rhs.startswith('"') and raw_rhs.endswith('"')) or (raw_rhs.startswith("'") and raw_rhs.endswith("'")):
            rhs = raw_rhs[1:-1]
            return str(lhs) == rhs
        # number
        if re.match(r"^-?\d+(\.\d+)?$", raw_rhs):
            try:
                rhs_num = float(raw_rhs) if "." in raw_rhs else int(raw_rhs)
            except Exception:
                return False
            try:
                return float(lhs) == float(rhs_num)
            except Exception:
                return False
        # fallback: raw compare
        return str(lhs) == raw_rhs

    if isinstance(any_list, list):
        return any(eval_expr(str(x)) for x in any_list)
    if isinstance(all_list, list):
        return all(eval_expr(str(x)) for x in all_list)
    return True


def load_pipeline(pipeline_path: Path) -> Dict[str, Any]:
    """
    Load pipeline.yaml.

    Supports either:
    - a single YAML mapping, OR
    - a 2-document "front matter" style:
        --- (meta)
        ---
        (body)
    """
    text = _read_text(pipeline_path)
    docs = list(yaml.safe_load_all(text))
    if not docs:
        return {}
    if len(docs) == 1:
        data = docs[0] or {}
        if not isinstance(data, dict):
            raise ValueError("pipeline.yaml must parse to a mapping")
        return data

    merged: Dict[str, Any] = {}
    for d in docs:
        if isinstance(d, dict):
            merged.update(d)
    return merged



def resolve_pipeline_path(pipeline_arg: str | None) -> Path:
    """Resolve a pipeline path.

    - If `pipeline_arg` is None, defaults to ROOT / 'pipeline.yaml'
    - If `pipeline_arg` is relative, it is interpreted relative to ROOT.
    """
    if not pipeline_arg:
        return ROOT / "pipeline.yaml"
    p = Path(pipeline_arg)
    if not p.is_absolute():
        p = ROOT / p
    return p

def load_profile(profile_id: str) -> Dict[str, Any]:
    prof_dir = ROOT / "profiles" / profile_id
    if not prof_dir.exists():
        raise FileNotFoundError(f"Profile not found: {profile_id}")
    profile_yaml = prof_dir / "profile.yaml"
    commands_yaml = prof_dir / "commands.yaml"
    conventions_md = prof_dir / "conventions.md"
    out: Dict[str, Any] = {"id": profile_id, "dir": str(prof_dir)}
    out["profile_yaml_path"] = str(profile_yaml)
    out["commands_yaml_path"] = str(commands_yaml)
    out["conventions_md_path"] = str(conventions_md)
    out["profile_yaml"] = _read_text(profile_yaml) if profile_yaml.exists() else ""
    out["commands_yaml"] = _read_text(commands_yaml) if commands_yaml.exists() else ""
    out["conventions_md"] = _read_text(conventions_md) if conventions_md.exists() else ""
    return out


def load_runner(runner_id: str) -> Dict[str, Any]:
    r_path = ROOT / "runners" / f"{runner_id}.md"
    return {"id": runner_id, "path": str(r_path), "content": _read_text(r_path) if r_path.exists() else ""}


def _is_example_path(path: Path, root: Path) -> bool:
    try:
        rel_parts = path.relative_to(root).parts
    except ValueError:
        return False
    return "examples" in rel_parts


def list_overlays() -> List[str]:
    """List overlay ids under core/overlays.

    Overlays may be nested in subdirectories. The returned overlay id is the relative
    path from core/overlays without the .md suffix, using forward slashes.

    Examples:
      - quality/complexity_assessment
      - sprint/bounded_sprints_lanes
      - task/research_discovery
    """
    overlay_dir = ROOT / "core" / "overlays"
    if not overlay_dir.exists():
        return []
    names: List[str] = []
    for p in overlay_dir.rglob("*.md"):
        if _is_example_path(p, overlay_dir):
            continue
        rel = p.relative_to(overlay_dir).with_suffix("")
        names.append(str(rel).replace("\\", "/"))
    return sorted(set(names))


def load_overlay(name: str) -> Tuple[str, str]:
    """Load an overlay by id (relative path under core/overlays, without .md)."""
    overlay_dir = ROOT / "core" / "overlays"
    # Allow callers to pass either "foo/bar" or "foo/bar.md"
    rel = name[:-3] if name.endswith(".md") else name
    p = (overlay_dir / f"{rel}.md").resolve()
    # Basic path traversal guard
    if overlay_dir.resolve() not in p.parents and p != overlay_dir.resolve():
        raise ValueError(f"Invalid overlay path: {name}")
    if not p.exists():
        raise FileNotFoundError(f"Overlay not found: {name}")
    return str(p), _read_text(p)


def stage_compiled_path(stage_id: str) -> Path:
    safe = stage_id.replace("/", "_").replace(":", "_")
    return DIST_DIR / f"{safe}.md"


def compile_stage_prompt(
    stage_id: str,
    stage_file: Path,
    variables: Dict[str, Any],
    overlays: List[str],
    include_profile_pack: bool = True,
) -> str:
    fm, body = parse_markdown_front_matter(stage_file)

    # Work level controls optional scoped blocks in included markdown.
    # Default to L1 (project/planning) if not specified.
    work_level = str(
        fm.get("work_level")
        or fm.get("level")
        or variables.get("work_level")
        or "L1"
    ).strip()
    if not work_level:
        work_level = "L1"

    # Ensure compiled prompt shows stage work_level in Run Variables.
    stage_vars = dict(variables)
    stage_vars["work_level"] = work_level
    variables = stage_vars

    def _scoped(txt: str) -> str:
        return filter_scoped_blocks(txt, work_level).rstrip() or "(empty)"

    title = fm.get("title") or stage_id
    desc = (fm.get("description") or "").strip()
    includes = fm.get("includes") or []
    inputs = fm.get("inputs") or {}
    gating = fm.get("gating") or {}
    outputs = fm.get("outputs") or {}

    # Resolve variables
    resolved_includes: List[str] = []
    for inc in includes if isinstance(includes, list) else []:
        if not isinstance(inc, str):
            continue
        resolved_includes.append(_substitute_vars(inc, variables))

    # Build prompt
    lines: List[str] = []
    lines.append(f"# {stage_id} — {title}")
    if desc:
        lines.append("")
        lines.append(desc)
    lines.append("\n\n## Run Variables\n")
    for k in sorted(variables.keys()):
        lines.append(f"- {k}: {variables[k]}")
    lines.append("")

    if include_profile_pack:
        prof_id = str(variables.get("profile", "") or "")
        runner_id = str(variables.get("runner", "") or "")
        lines.append("\n## Selected Runner\n")
        r = load_runner(runner_id) if runner_id else {"id": "", "content": ""}
        lines.append(f"### runners/{runner_id}.md\n")
        lines.append(_scoped(r.get("content", "")))
        lines.append("")
        lines.append("\n## Selected Profile\n")
        if prof_id:
            prof = load_profile(prof_id)
            lines.append(f"### profiles/{prof_id}/profile.yaml\n")
            lines.append(_scoped(prof.get("profile_yaml", "")))
            lines.append("")
            lines.append(f"### profiles/{prof_id}/commands.yaml\n")
            lines.append(_scoped(prof.get("commands_yaml", "")))
            lines.append("")
            lines.append(f"### profiles/{prof_id}/conventions.md\n")
            lines.append(_scoped(prof.get("conventions_md", "")))
        else:
            lines.append("(no profile selected)")
        lines.append("")

    if overlays:
        lines.append("\n## Overlays\n")
        for ov in overlays:
            try:
                ov_path, ov_txt = load_overlay(ov)
            except Exception as e:
                lines.append(f"### (missing overlay: {ov})\n\n{e}\n")
                continue
            rel = Path(ov_path).relative_to(ROOT)
            lines.append(f"### {rel}\n")
            lines.append(_scoped(ov_txt))
            lines.append("")

    # Includes
    lines.append("\n## Includes\n")
    for inc in resolved_includes:
        p = ROOT / inc
        lines.append(f"### {inc}\n")
        if p.exists():
            lines.append(_scoped(_read_text(p)))
        else:
            lines.append("(missing)")
        lines.append("")

    # Library inputs
    lib_inputs = (inputs.get("library") or []) if isinstance(inputs, dict) else []
    if lib_inputs:
        lines.append("\n## Library Inputs\n")
        for item in lib_inputs:
            if isinstance(item, dict):
                path = item.get("path")
            else:
                path = item
            if not path:
                continue
            if isinstance(path, str):
                path_s = _substitute_vars(path, variables)

                # TEST MODE directive switching for Charter stage
                # If test_mode is true and stage is charter interview, prefer the TEST_MODE directive.
                if stage_id == "stage.05_charter_interview":
                    test_mode = _coerce_bool(variables.get("test_mode")) or False
                    normal = "core/library/charter/charter_gen_directive.md"
                    test = "core/library/charter/charter_gen_directive_TEST_MODE.md"

                    if test_mode and path_s == normal:
                        path_s = test
                    elif (not test_mode) and path_s == test:
                        # If both directives are present in inputs, include only the selected one
                        continue

                p = ROOT / path_s
                lines.append(f"### {path_s}\n")
                if p.exists():
                    lines.append(_scoped(_read_text(p)))
                else:
                    lines.append("(missing)")
                lines.append("")
    else:
        lines.append("\n## Library Inputs\n\n(none)\n")

    # Artifact inputs
    art_inputs = (inputs.get("artifacts") or []) if isinstance(inputs, dict) else []
    if art_inputs:
        lines.append("\n## Artifact Inputs\n")
        for item in art_inputs:
            if isinstance(item, dict):
                path = item.get("path")
                required = bool(item.get("required", False))
            else:
                path = item
                required = False
            if not path:
                continue
            path_s = _substitute_vars(str(path), variables)
            p = ROOT / path_s
            lines.append(f"### {path_s} {'(required)' if required else '(optional)'}\n")
            if p.exists():
                lines.append(_scoped(_read_text(p)))
            else:
                lines.append("(missing)")
            lines.append("")
    else:
        lines.append("\n## Artifact Inputs\n\n(none)\n")

    # Outputs
    lines.append("\n## Outputs\n")
    # Artifacts
    lines.append("\n### Artifacts\n")
    out_art = outputs.get("artifacts") if isinstance(outputs, dict) else None
    art_paths: List[str] = []
    if isinstance(out_art, list):
        for o in out_art:
            if isinstance(o, dict) and o.get("path"):
                art_paths.append(_substitute_vars(str(o["path"]), variables))
            elif isinstance(o, str):
                art_paths.append(_substitute_vars(o, variables))
    if art_paths:
        for op in art_paths:
            lines.append(f"- {op}")
    else:
        lines.append("(none declared)")
    lines.append("")

    # Repo files (optional, but commonly used for canonical docs in the repo root)
    lines.append("\n### Repo Files\n")
    out_repo = outputs.get("repo_files") if isinstance(outputs, dict) else None
    repo_paths: List[str] = []
    if isinstance(out_repo, list):
        for o in out_repo:
            if isinstance(o, dict) and o.get("path"):
                repo_paths.append(_substitute_vars(str(o["path"]), variables))
            elif isinstance(o, str):
                repo_paths.append(_substitute_vars(o, variables))
    if repo_paths:
        for rp in repo_paths:
            lines.append(f"- {rp}")
    else:
        lines.append("(none declared)")
    lines.append("")

    # Gating notes
    notes = gating.get("notes") if isinstance(gating, dict) else None
    if isinstance(notes, list) and notes:
        lines.append("\n## Gating Notes\n")
        for n in notes:
            lines.append(f"- {n}")
        lines.append("")
    elif notes:
        lines.append("\n## Gating Notes\n")
        lines.append(str(notes))
        lines.append("")

    # Stage body (rarely used)
    if body.strip():
        lines.append("\n## Stage Body\n")
        lines.append(body.strip())
        lines.append("")

    return "\n".join(lines).strip() + "\n"


def parse_file_blocks(text: str) -> Dict[str, str]:
    """
    Parse multi-file output blocks:
      --- FILE: path ---
      <content>
    Returns mapping path->content.
    """
    out: Dict[str, str] = {}
    lines = text.splitlines()
    i = 0
    current_path: Optional[str] = None
    buf: List[str] = []

    def flush() -> None:
        nonlocal current_path, buf
        if current_path is not None:
            out[current_path] = "\n".join(buf).strip() + "\n"
        current_path = None
        buf = []

    header_re = re.compile(r"^---\s*FILE:\s*(.+?)\s*---\s*$")
    while i < len(lines):
        m = header_re.match(lines[i])
        if m:
            flush()
            current_path = m.group(1).strip()
            buf = []
        else:
            if current_path is not None:
                buf.append(lines[i])
        i += 1
    flush()
    return out



def _tty_input(prompt: str) -> str:
    """
    Read a line from the controlling TTY if available, falling back to stdin.
    This keeps capture mode usable (stdin may be closed after Ctrl-D).
    """
    try:
        with open("/dev/tty", "r") as tty_in:
            sys.stdout.write(prompt)
            sys.stdout.flush()
            return tty_in.readline().strip()
    except Exception:
        # fallback
        try:
            return input(prompt).strip()
        except EOFError:
            return ""


def _prompt_bool(name: str, default: bool) -> bool:
    d = "Y/n" if default else "y/N"
    ans = _tty_input(f"Set {name}? [{d}]: ").strip().lower()
    if not ans:
        return default
    if ans in {"y", "yes", "true", "1"}:
        return True
    if ans in {"n", "no", "false", "0"}:
        return False
    return default


def ensure_outputs_exist(output_paths: List[Path]) -> None:
    for p in output_paths:
        p.parent.mkdir(parents=True, exist_ok=True)


def cmd_list(args: argparse.Namespace) -> int:
    pipeline_path = resolve_pipeline_path(getattr(args, "pipeline", None))
    pipeline = load_pipeline(pipeline_path)
    stages = pipeline.get("stages") or []
    if not isinstance(stages, list):
        print("pipeline.yaml: 'stages' must be a list", file=sys.stderr)
        return 2
    print("Stages:")
    for s in stages:
        if not isinstance(s, dict):
            continue
        sid = s.get("id")
        f = s.get("file")
        print(f"- {sid}  ({f})")
    return 0


def _load_state(path: Path) -> Dict[str, Any]:
    if not path.exists():
        return {}
    try:
        data = yaml.safe_load(_read_text(path)) or {}
        return data if isinstance(data, dict) else {}
    except Exception:
        return {}


def _save_state(path: Path, state: Dict[str, Any]) -> None:
    _write_text(path, yaml.safe_dump(state, sort_keys=True))


def _merge_vars(base: Dict[str, Any], overrides: Dict[str, Any]) -> Dict[str, Any]:
    merged = dict(base)
    merged.update({k: v for k, v in overrides.items() if v is not None})
    return merged



def _update_state_after_capture(stage_id: str, stage_entry: Dict[str, Any], wrote_paths: List[str], output_text: str) -> None:
    """
    Update artifacts/_harness_state.yaml after capturing a stage output.

    - Applies pipeline-level `sets:` (e.g., needs_project_context)
    - Updates convenience refs (charter_ref, project_context_ref) based on outputs
    """
    state_path = ARTIFACTS_DIR / "_harness_state.yaml"
    state = _load_state(state_path)

    # convenience refs
    for p in wrote_paths:
        if p.endswith("artifacts/charter/CHARTER.md"):
            state["charter_ref"] = "artifacts/charter/CHARTER.md"
            # Basic heuristic: if the charter contains obvious unknown markers, track it.
            markers = ["TBD", "UNKNOWN", "Unknown", "TODO", "??"]
            state["charter_gaps_detected"] = any(m in output_text for m in markers)
        if p.endswith("artifacts/project_context/PROJECT_CONTEXT.md"):
            state["project_context_ref"] = "artifacts/project_context/PROJECT_CONTEXT.md"
        if p.endswith("artifacts/base/BASE_CONTEXT.md"):
            state["base_context_ref"] = "artifacts/base/BASE_CONTEXT.md"

    # apply `sets` variables (usually booleans)
    sets = stage_entry.get("sets") or []
    if isinstance(sets, list):
        for name in sets:
            if not isinstance(name, str):
                continue
            # If the value is already set, don't re-prompt.
            # This makes repeat captures and non-interactive runs less disruptive.
            if name in state:
                continue
            # heuristic defaults
            guess = False
            if name == "needs_project_context":
                # If output contains unknown markers, default to True.
                markers = ["TBD", "UNKNOWN", "Unknown", "TODO", "??"]
                guess = any(m in output_text for m in markers)
            val = _prompt_bool(name, guess)
            state[name] = val

    _save_state(state_path, state)


def cmd_compile(args: argparse.Namespace) -> int:
    pipeline_path = resolve_pipeline_path(getattr(args, "pipeline", None))
    pipeline = load_pipeline(pipeline_path)
    defaults = pipeline.get("defaults") or {}
    if not isinstance(defaults, dict):
        defaults = {}

    # State (optional)
    state_path = ARTIFACTS_DIR / "_harness_state.yaml"
    state = _load_state(state_path)

    now_utc = args.now_utc or state.get("now_utc") or _dt.datetime.utcnow().replace(microsecond=0).isoformat() + "Z"

    var_overrides: Dict[str, Any] = {
        "runner": args.runner or state.get("runner") or defaults.get("runner"),
        "profile": args.profile or state.get("profile") or defaults.get("profile"),
        "repo_root": args.repo_root or state.get("repo_root") or ".",
        "enable_complexity": args.enable_complexity if args.enable_complexity is not None else state.get("enable_complexity", defaults.get("enable_complexity", False)),
        "needs_project_context": args.needs_project_context if args.needs_project_context is not None else state.get("needs_project_context", False),
        "charter_gaps_detected": state.get("charter_gaps_detected", False),
        "test_mode": args.test_mode if args.test_mode is not None else state.get("test_mode", False),
        "now_utc": now_utc,
        "project_name": args.project_name or state.get("project_name", ""),
        "owner": args.owner or state.get("owner", ""),
        "team": args.team or state.get("team", ""),
        "repo_or_project_ref": args.repo_or_project_ref or state.get("repo_or_project_ref", ""),
        "charter_ref": state.get("charter_ref", ""),
        "project_context_ref": state.get("project_context_ref", ""),
        "release_id": args.release_id or state.get("release_id", "release-001"),
        "release_type": args.release_type or state.get("release_type", "minor"),
        "sprint_id": args.sprint_id or state.get("sprint_id", "SPRINT-SEQ-0001"),
        "sprint_slot": args.sprint_slot or state.get("sprint_slot", "slot-1"),
        "prev_sprint_id": args.prev_sprint_id or state.get("prev_sprint_id", ""),
    }

    # overlays
    overlays: List[str] = []
    if args.overlays:
        overlays.extend([x.strip() for x in args.overlays.split(",") if x.strip()])
    # auto overlay: complexity assessment if enabled
    if var_overrides.get("enable_complexity"):
        candidates = ["quality/complexity_assessment", "complexity_assessment"]
        if not any(c in overlays for c in candidates):
            avail = set(list_overlays())
            chosen = next((c for c in candidates if c in avail), None)
            if chosen:
                overlays.append(chosen)

    DIST_DIR.mkdir(parents=True, exist_ok=True)

    # Determine which stages
    stages = pipeline.get("stages") or []
    if not isinstance(stages, list):
        raise ValueError("pipeline.yaml 'stages' must be a list")

    only = args.only
    until = args.until

    selected: List[Dict[str, Any]] = []
    for s in stages:
        if not isinstance(s, dict):
            continue
        sid = s.get("id")
        if not sid:
            continue
        selected.append(s)
        if until and sid == until:
            break
    if only:
        selected = [s for s in selected if s.get("id") == only]

    written: List[str] = []
    for s in selected:
        sid = s.get("id")
        stage_path = ROOT / str(s.get("file"))
        if not stage_path.exists():
            print(f"[WARN] Stage file missing: {stage_path}", file=sys.stderr)
            continue
        fm, _ = parse_markdown_front_matter(stage_path)

        # apply stage activation (from stage front matter)
        activation_ok = eval_activation(fm.get("activation") or {}, var_overrides)
        if not activation_ok and not args.force:
            # still write a compiled file explaining skip
            skip_txt = f"# {sid} — SKIPPED (activation not satisfied)\n\nActivation conditions not met for variables.\n"
            out_path = stage_compiled_path(sid)
            _write_text(out_path, skip_txt)
            written.append(str(out_path.relative_to(ROOT)))
            continue

        prompt = compile_stage_prompt(sid, stage_path, var_overrides, overlays, include_profile_pack=True)
        out_path = stage_compiled_path(sid)
        _write_text(out_path, prompt)
        written.append(str(out_path.relative_to(ROOT)))

    # update state with any CLI provided vars
    state.update({k: var_overrides[k] for k in var_overrides})
    _save_state(state_path, state)

    print("Wrote:")
    for p in written:
        print(f"- {p}")
    return 0


def cmd_capture(args: argparse.Namespace) -> int:
    """
    Capture output for a single stage and write artifact files.

    Usage patterns:
    - Single-file stages: paste the complete file contents.
    - Multi-file stages: paste `--- FILE: <path> ---` blocks.

    After writing artifacts, this will update `artifacts/_harness_state.yaml` for:
    - pipeline `sets:` variables (prompted, with heuristics)
    - convenience refs (charter_ref, project_context_ref, base_context_ref)
    """
    pipeline_path = resolve_pipeline_path(getattr(args, "pipeline", None))
    pipeline = load_pipeline(pipeline_path)
    stages = pipeline.get("stages") or []
    stage_map = {s.get("id"): s for s in stages if isinstance(s, dict) and s.get("id")}
    if args.stage_id not in stage_map:
        print(f"Unknown stage id: {args.stage_id}", file=sys.stderr)
        return 2

    stage_entry = stage_map[args.stage_id]
    stage_file = ROOT / str(stage_entry.get("file"))
    if not stage_file.exists():
        print(f"Missing stage file: {stage_file}", file=sys.stderr)
        return 2

    # Load state for variable substitution in output paths (e.g., ${repo_root}/...).
    defaults = pipeline.get("defaults") or {}
    if not isinstance(defaults, dict):
        defaults = {}
    state_path = ARTIFACTS_DIR / "_harness_state.yaml"
    state = _load_state(state_path)
    # Use full state for variable substitution in output paths (e.g., ${release_id}).
    # Provide a few safe fallbacks so capture can work even if compile wasn't run first.
    variables: Dict[str, Any] = dict(state)
    variables.setdefault("runner", state.get("runner") or defaults.get("runner"))
    variables.setdefault("profile", state.get("profile") or defaults.get("profile"))
    variables.setdefault("repo_root", state.get("repo_root") or ".")
    variables.setdefault("release_id", state.get("release_id") or "release-001")
    variables.setdefault("release_type", state.get("release_type") or "minor")
    variables.setdefault("sprint_id", state.get("sprint_id") or "SPRINT-SEQ-0001")
    variables.setdefault("sprint_slot", state.get("sprint_slot") or "slot-1")
    variables.setdefault("prev_sprint_id", state.get("prev_sprint_id") or "")

    fm, _ = parse_markdown_front_matter(stage_file)
    outputs = fm.get("outputs") or {}

    # Declared artifact outputs
    out_art = outputs.get("artifacts") if isinstance(outputs, dict) else None
    art_paths: List[str] = []
    if isinstance(out_art, list):
        for o in out_art:
            if isinstance(o, dict) and o.get("path"):
                art_paths.append(_substitute_vars(str(o["path"]), variables))
            elif isinstance(o, str):
                art_paths.append(_substitute_vars(o, variables))

    # Declared repo file outputs (canonical docs written into the repo itself)
    out_repo = outputs.get("repo_files") if isinstance(outputs, dict) else None
    repo_out: List[Tuple[str, bool]] = []  # (path, required)
    if isinstance(out_repo, list):
        for o in out_repo:
            if isinstance(o, dict) and o.get("path"):
                repo_out.append((_substitute_vars(str(o["path"]), variables), bool(o.get("required", False))))
            elif isinstance(o, str):
                repo_out.append((_substitute_vars(o, variables), False))

    text = sys.stdin.read()
    if not art_paths:
        print("Stage declares no artifact outputs; nothing to write.", file=sys.stderr)
        return 2

    wrote_paths: List[str] = []

    # Single-file stage: write the same content to all declared artifact outputs
    # and any declared repo files.
    if len(art_paths) == 1:
        content = text.strip() + "\n"

        # Artifacts
        for ap in art_paths:
            out_path = ROOT / ap
            _write_text(out_path, content)
            rel = str(out_path.relative_to(ROOT))
            wrote_paths.append(rel)
            print(f"Wrote {rel}")

        # Repo files
        for rp, _required in repo_out:
            rp_path = ROOT / rp
            _write_text(rp_path, content)
            rel = str(rp_path.relative_to(ROOT))
            wrote_paths.append(rel)
            print(f"Wrote {rel}")

        _update_state_after_capture(args.stage_id, stage_entry, wrote_paths, text)
        return 0

    blocks = parse_file_blocks(text)
    if not blocks:
        print("Expected multi-file output blocks (--- FILE: ... ---), but none were found.", file=sys.stderr)
        return 2

    # Write only files that are declared artifact outputs (ignore extras)
    declared_art = set(art_paths)
    wrote_art: Dict[str, str] = {}
    for p, content in blocks.items():
        if p not in declared_art:
            continue
        _write_text(ROOT / p, content)
        wrote_paths.append(p)
        wrote_art[p] = content
        print(f"Wrote {p}")

    missing_art = declared_art - set(wrote_art.keys())
    if missing_art:
        print("Missing declared artifact outputs:", file=sys.stderr)
        for m in sorted(missing_art):
            print(f"- {m}", file=sys.stderr)
        return 2

    # Also write any declared repo files.
    # If the model didn't emit a repo file block (common), we try to copy from a
    # matching artifact output with the same filename.
    for rp, required in repo_out:
        if rp in blocks:
            _write_text(ROOT / rp, blocks[rp])
            wrote_paths.append(rp)
            print(f"Wrote {rp}")
            continue

        # Copy-from heuristic: match by basename.
        base = Path(rp).name
        src_path = next((ap for ap in art_paths if Path(ap).name == base and ap in wrote_art), None)
        if src_path:
            _write_text(ROOT / rp, wrote_art[src_path])
            wrote_paths.append(rp)
            print(f"Wrote {rp} (copied from {src_path})")
            continue

        if required:
            print(f"Required repo file output missing and could not be inferred: {rp}", file=sys.stderr)
            return 2

    if not wrote_paths:
        print("No declared output files were found in the pasted blocks.", file=sys.stderr)
        return 2

    _update_state_after_capture(args.stage_id, stage_entry, wrote_paths, text)
    return 0



def cmd_run(args: argparse.Namespace) -> int:
    """
    Convenience: compile a single stage prompt and then immediately capture its output.

    Workflow:
    1) This writes dist/<stage>.md
    2) You copy/paste that dist prompt into your LLM
    3) Paste the LLM output back into this command (end with Ctrl-D)
    4) Harness writes artifacts and updates state
    """
    pipeline_path = resolve_pipeline_path(getattr(args, "pipeline", None))
    pipeline = load_pipeline(pipeline_path)
    defaults = pipeline.get("defaults") or {}
    if not isinstance(defaults, dict):
        defaults = {}

    # State
    state_path = ARTIFACTS_DIR / "_harness_state.yaml"
    state = _load_state(state_path)

    now_utc = args.now_utc or state.get("now_utc") or _dt.datetime.utcnow().replace(microsecond=0).isoformat() + "Z"

    var_overrides: Dict[str, Any] = {
        "runner": args.runner or state.get("runner") or defaults.get("runner"),
        "profile": args.profile or state.get("profile") or defaults.get("profile"),
        "repo_root": args.repo_root or state.get("repo_root") or ".",
        "enable_complexity": args.enable_complexity if args.enable_complexity is not None else state.get("enable_complexity", defaults.get("enable_complexity", False)),
        "needs_project_context": args.needs_project_context if args.needs_project_context is not None else state.get("needs_project_context", False),
        "charter_gaps_detected": state.get("charter_gaps_detected", False),
        "test_mode": args.test_mode if args.test_mode is not None else state.get("test_mode", False),
        "now_utc": now_utc,
        "project_name": args.project_name or state.get("project_name", ""),
        "owner": args.owner or state.get("owner", ""),
        "team": args.team or state.get("team", ""),
        "repo_or_project_ref": args.repo_or_project_ref or state.get("repo_or_project_ref", ""),
        "charter_ref": state.get("charter_ref", ""),
        "project_context_ref": state.get("project_context_ref", ""),
        "release_id": args.release_id or state.get("release_id", "release-001"),
        "release_type": args.release_type or state.get("release_type", "minor"),
        "sprint_id": args.sprint_id or state.get("sprint_id", "SPRINT-SEQ-0001"),
        "sprint_slot": args.sprint_slot or state.get("sprint_slot", "slot-1"),
        "prev_sprint_id": args.prev_sprint_id or state.get("prev_sprint_id", ""),
    }

    # overlays
    overlays: List[str] = []
    if args.overlays:
        overlays.extend([x.strip() for x in args.overlays.split(",") if x.strip()])

    if var_overrides.get("enable_complexity"):
        candidates = ["quality/complexity_assessment", "complexity_assessment"]
        if not any(c in overlays for c in candidates):
            avail = set(list_overlays())
            chosen = next((c for c in candidates if c in avail), None)
            if chosen:
                overlays.append(chosen)

    # Find stage
    stages = pipeline.get("stages") or []
    stage_entry = None
    for s in stages:
        if isinstance(s, dict) and s.get("id") == args.stage_id:
            stage_entry = s
            break
    if stage_entry is None:
        print(f"Unknown stage id: {args.stage_id}", file=sys.stderr)
        return 2

    stage_path = ROOT / str(stage_entry.get("file"))
    if not stage_path.exists():
        print(f"Stage file missing: {stage_path}", file=sys.stderr)
        return 2

    fm, _ = parse_markdown_front_matter(stage_path)
    activation_ok = eval_activation(fm.get("activation") or {}, var_overrides)
    if not activation_ok and not args.force:
        print(f"Stage {args.stage_id} skipped (activation not satisfied). Use --force to override.")
        return 3

    DIST_DIR.mkdir(parents=True, exist_ok=True)
    prompt = compile_stage_prompt(args.stage_id, stage_path, var_overrides, overlays, include_profile_pack=True)
    out_path = stage_compiled_path(args.stage_id)
    _write_text(out_path, prompt)

    # persist state
    state.update({k: var_overrides[k] for k in var_overrides})
    _save_state(state_path, state)

    print(f"Compiled: {out_path.relative_to(ROOT)}")
    print("1) Copy/paste that prompt into your LLM.")
    print("2) Paste the LLM output here, then press Ctrl-D.\n")

    # Capture using existing logic (reads stdin)
    cap_args = argparse.Namespace(stage_id=args.stage_id, pipeline=getattr(args, "pipeline", None))
    return cmd_capture(cap_args)

def cmd_overlays(args: argparse.Namespace) -> int:
    for name in list_overlays():
        print(name)
    return 0


def main(argv: Optional[List[str]] = None) -> int:
    parser = argparse.ArgumentParser(
        prog="harness.py",
        formatter_class=argparse.RawTextHelpFormatter,
        description=HELP_BANNER,
    )
    sub = parser.add_subparsers(dest="cmd", required=True)

    p_list = sub.add_parser("list", help="List pipeline stages")
    p_list.add_argument("--pipeline", help="Pipeline YAML to use (relative to handbook root, or absolute path)")
    p_list.set_defaults(func=cmd_list)

    p_ov = sub.add_parser("overlays", help="List available overlays")
    p_ov.set_defaults(func=cmd_overlays)

    p_compile = sub.add_parser("compile", help="Compile prompts into dist/")
    p_compile.add_argument("--pipeline", help="Pipeline YAML to use (relative to handbook root, or absolute path)")
    p_compile.add_argument("--only", help="Compile only one stage id")
    p_compile.add_argument("--until", help="Compile stages up to and including this stage id")
    p_compile.add_argument("--force", action="store_true", help="Compile even if activation conditions fail")
    p_compile.add_argument("--runner", help="Override runner id")
    p_compile.add_argument("--profile", help="Override profile id")
    p_compile.add_argument("--repo-root", dest="repo_root", help="Repo root path (logical)")
    p_compile.add_argument("--project-name", dest="project_name", help="Project name")
    p_compile.add_argument("--repo-or-project-ref", dest="repo_or_project_ref", help="Repo/project reference")
    p_compile.add_argument("--release-id", dest="release_id", help="Release identifier (used in release/sprint output paths)")
    p_compile.add_argument("--release-type", dest="release_type", help="Release type (e.g., minor, major, hardening)")
    p_compile.add_argument("--sprint-id", dest="sprint_id", help="Sprint identifier (used in sprint output paths)")
    p_compile.add_argument("--sprint-slot", dest="sprint_slot", help="Sprint slot id within the release (e.g., slot-1)")
    p_compile.add_argument("--prev-sprint-id", dest="prev_sprint_id", help="Previous sprint identifier (optional; used for sprint planning inputs)")
    p_compile.add_argument("--owner", help="Owner")
    p_compile.add_argument("--team", help="Team")
    p_compile.add_argument("--now-utc", dest="now_utc", help="Override NOW_UTC (ISO8601Z)")
    p_compile.add_argument("--enable-complexity", dest="enable_complexity", type=lambda x: x.lower()=="true", nargs="?", const=True, help="Enable complexity overlay")
    p_compile.add_argument("--needs-project-context", dest="needs_project_context", type=lambda x: x.lower()=="true", nargs="?", const=True, help="Force needs_project_context")
    p_compile.add_argument("--test-mode", dest="test_mode", type=lambda x: x.lower()=="true", nargs="?", const=True, help="Enable test_mode (e.g., Charter TEST MODE directive)")

    p_compile.add_argument("--overlays", help="Comma-separated overlays to include")
    p_compile.set_defaults(func=cmd_compile)


    p_run = sub.add_parser("run", help="Compile + capture a single stage (interactive paste)")
    p_run.add_argument("--pipeline", help="Pipeline YAML to use (relative to handbook root, or absolute path)")
    p_run.add_argument("stage_id", help="Stage id to run")
    p_run.add_argument("--force", action="store_true", help="Run even if activation conditions fail")
    p_run.add_argument("--runner", help="Override runner id")
    p_run.add_argument("--profile", help="Override profile id")
    p_run.add_argument("--repo-root", dest="repo_root", help="Repo root path (logical)")
    p_run.add_argument("--project-name", dest="project_name", help="Project name")
    p_run.add_argument("--repo-or-project-ref", dest="repo_or_project_ref", help="Repo/project reference")
    p_run.add_argument("--release-id", dest="release_id", help="Release identifier (used in release/sprint output paths)")
    p_run.add_argument("--release-type", dest="release_type", help="Release type (e.g., minor, major, hardening)")
    p_run.add_argument("--sprint-id", dest="sprint_id", help="Sprint identifier (used in sprint output paths)")
    p_run.add_argument("--sprint-slot", dest="sprint_slot", help="Sprint slot id within the release (e.g., slot-1)")
    p_run.add_argument("--prev-sprint-id", dest="prev_sprint_id", help="Previous sprint identifier (optional; used for sprint planning inputs)")
    p_run.add_argument("--owner", help="Owner")
    p_run.add_argument("--team", help="Team")
    p_run.add_argument("--now-utc", dest="now_utc", help="Override NOW_UTC (ISO8601Z)")
    p_run.add_argument("--enable-complexity", dest="enable_complexity", type=lambda x: x.lower()=="true", nargs="?", const=True, help="Enable complexity overlay")
    p_run.add_argument("--needs-project-context", dest="needs_project_context", type=lambda x: x.lower()=="true", nargs="?", const=True, help="Force needs_project_context")
    p_run.add_argument("--test-mode", dest="test_mode", type=lambda x: x.lower()=="true", nargs="?", const=True, help="Enable test_mode (e.g., Charter TEST MODE directive)")
    p_run.add_argument("--overlays", help="Comma-separated overlays to include")
    p_run.set_defaults(func=cmd_run)

    p_capture = sub.add_parser("capture", help="Capture model output for a stage and write artifacts")
    p_capture.add_argument("--pipeline", help="Pipeline YAML to use (relative to handbook root, or absolute path)")
    p_capture.add_argument("stage_id", help="Stage id to capture output for")
    p_capture.set_defaults(func=cmd_capture)

    args = parser.parse_args(argv)
    return int(args.func(args))


if __name__ == "__main__":
    raise SystemExit(main())
