# Runner: codex-cli

This runner assumes the agent can:
- run shell commands in the repo
- read/write files
- show command evidence (cmd, exit code, tail)

## Command execution rules
- Use the selected profile’s command keys (e.g., `commands.tests`) as the source of truth.
- Capture evidence for every gate you claim passed (see Evidence Policy).

## File edits
- Prefer minimal diffs.
- Keep changes within slice scope.
- If a change introduces/changes env vars, services, ports, or runtime assumptions:
  - add new named environments or capabilities to `.handbook/project/environment.yaml` before a work contract references them

## Output discipline
- For “document stages”, output only the document content (no extra commentary).
- For multi-artifact stages, output `--- FILE: <path> ---` blocks exactly.
