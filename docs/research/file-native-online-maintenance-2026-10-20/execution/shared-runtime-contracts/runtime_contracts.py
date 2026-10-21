"""SOURCE ONLY shared-runtime-contracts-v1 under claim042 (one80KiB file).
Stdlib-only module load: no private import/native/thread/write/engine action.
Direct owned API: reader_capacity, derive_whole_forecast, make_runner,
seal_final_provisional, sealed_phase_evidence. 035 owns ALL typed formulas and
encode_private_record. Windows-only frozen1600-history; native158unchanged.
Native source-bound PORT_METHODS supplies primitives, never whole-phase PASS.
"""

import copy
import hashlib
import importlib.util
import inspect
import json
import math
from pathlib import Path
import threading
import time

API = "shared-runtime-contracts-v1"
PORT_API = "history-ports-v1"
CAPTURE = "single-raw-ref-v1"
BODY_SCHEMA = "history-final-provisional-v1"
HISTORICAL_GUARD = "50ecc7c40c1cdda6493817261b5755fe2d823fdae271118698828c39a0400378"
CUTOFF = 1792581014
MIB = 1024 * 1024
MAX_CONTROL = 4096
MAX_PHASE = 2 * MIB
CATEGORIES = ("early", "middle", "recent", "missing", "cancelled", "lostreply")
ROOTS = {
    "H1": "C:/file-native-online-matrix-20261020/history/H1000-baseline/disposable/store",
    "H10": "C:/file-native-online-matrix-20261020/history/authentic-N128/disposable/store",
}
ARTIFACTS = (
    "runtime_source", "profile_source", "profile_review", "ports_source",
    "ports_review", "paced_source", "paced_review", "guard_source",
    "guard_schema", "guard_review", "clock_source", "clock_review",
    "finalizer_source", "finalizer_review", "history_source", "budget", "phase_go",
)
BINDING_KEYS = (
    "version", "platform", "history", "runtime_go", "paced_schema",
    "profile_config", "reader_policy", "case_catalog", *ARTIFACTS,
)
COUNTERS = (
    "rolling_publications_visited", "rolling_publication_body_opens",
    "rolling_discovery_metadata_bytes", "rolling_deferred_rechecked",
    "pack_metadata_stats", "pack_index_metadata_bytes", "pack_storage_bytes",
    "pack_decoded_bytes",
)
PORT_METHODS = (
    "bindings", "build_binding", "verify_case_catalog", "authorize_namespace",
    "read_owned", "write_once", "verify_ready", "clock_sample", "enter_phase",
    "leave_phase", "set_origin", "observe_started", "current_call_observation",
    "phase_call_observations", "abort_phase", "guard_prefix", "donor_authority",
    "seal_context", "append_phase_index", "read_reference", "case_call_pairs",
)
FULL_FORECAST_KEYS = (
    "source_pin", "capsule_sha256", "counts", "F05_bytes", "F05_files",
    "protocol_bytes", "protocol_files", "evidence_bytes", "evidence_files",
    "expanded_trace_sha256", "all_sink_membership_sha256", "profile_sha256",
)
_PROFILES = {}
_HISTORY = {}
BODY_KEYS = ("schema", "status", "phase_id", "plan_sha256", "source_pin",
    "binary_sha256", "budget_sha256", "outer_run_id", "source_profile_sha256",
    "runtime_sha256", "guard_sha256", "finalizer_sha256", "evidence", "summary",
    "owned_closure_ref", "checkpoint_ref", "case_closure_ref", "native_call_index_ref")
_PROFILE_LOCK = threading.RLock()


class RuntimeRefusal(RuntimeError):
    """Unreleased dependency, measured failure or mismatched owned evidence."""


def need(condition, reason):
    if not condition:
        raise RuntimeRefusal(reason)


def closed(value, keys, label):
    need(type(value) is dict and set(value) == set(keys), "closed:" + label)
    return value


def integer(value, low=0, high=(1 << 63) - 1):
    return type(value) is int and low <= value <= high


def digest(value, length=64):
    return type(value) is str and len(value) == length and all(c in "0123456789abcdef" for c in value)


def sha(raw):
    need(type(raw) is bytes, "hash-exact-bytes")
    return hashlib.sha256(raw).hexdigest()


def selector(value):
    closed(value, ("publication", "digest"), "selector")
    need(type(value["publication"]) is str and value["publication"].isascii()
         and 1 <= len(value["publication"]) <= 99 and digest(value["digest"]), "selector-values")
    return value


def _plain(path):
    path = Path(path)
    need(path.is_absolute() and ".." not in path.parts, "absolute-owned-path")
    for part in (path, *path.parents):
        if part.exists():
            st = part.lstat()
            need(not part.is_symlink() and not (getattr(st, "st_file_attributes", 0) & 0x400),
                 "source-or-control-link")
    return path


def artifact(spec, maximum=2 * MIB):
    closed(spec, ("path", "bytes", "sha256"), "artifact")
    need(integer(spec["bytes"], 1, maximum) and digest(spec["sha256"]), "artifact-bound")
    path = _plain(spec["path"])
    need(path.is_file() and path.stat().st_size == spec["bytes"], "artifact-size")
    with path.open("rb") as stream:
        raw = stream.read(spec["bytes"] + 1)
    need(len(raw) == spec["bytes"] and sha(raw) == spec["sha256"], "artifact-hash")
    return raw


def control_json(raw, maximum=MAX_CONTROL):
    need(type(raw) is bytes and len(raw) <= maximum, "control-byte-bound")
    def pairs(items):
        result = {}
        for key, value in items:
            need(key not in result, "duplicate-control-key")
            result[key] = value
        return result
    def invalid(value):
        raise RuntimeRefusal("nonfinite-control:" + value)
    return json.loads(raw, object_pairs_hook=pairs, parse_constant=invalid)


def reference(value, maximum=64 * MIB):
    need(type(value) is dict, "stored-reference")
    if value.get("schema") == CAPTURE:
        closed(value, ("schema", "path", "run", "call", "begin", "end", "frames", "bytes", "sha256"),
               "raw-call-reference")
        need(digest(value["run"], 32) and digest(value["call"], 32)
             and integer(value["begin"]) and integer(value["end"], value["begin"])
             and integer(value["frames"], 2), "raw-reference-span")
    else:
        need(set(value) <= {"path", "offset", "bytes", "sha256", "source_pin"}
             and {"path", "bytes", "sha256"} <= set(value), "record-reference-shape")
        need(integer(value.get("offset", 0)), "record-reference-offset")
    need(Path(value["path"]).is_absolute() and integer(value["bytes"], 1, maximum)
         and digest(value["sha256"]), "reference-values")
    return copy.deepcopy(value)


def _bindings(value, execution=False):
    closed(value, BINDING_KEYS, "runtime-bindings")
    need(value["version"] == 1 and type(value["version"]) is int
         and value["platform"] == "Windows", "Windows-history-only")
    need(value["paced_schema"] == "lookup-bench-concurrent-pacing-v1", "unreviewed-paced-ABI")
    need(type(value["history"]) is dict and digest(value["history"].get("source_pin"), 40),
         "history-final-pin")
    for name in ARTIFACTS:
        artifact(value[name])
    need(Path(value["runtime_source"]["path"]).resolve() == Path(__file__).resolve(),
         "exact-runtime-module")
    need(Path(value["profile_source"]["path"]).name == "long_source_profiles.py",
         "035-profile-owner")
    h = value["history"]
    for key in ("binary_sha256", "engine_manifest_sha256", "core_sha256", "runner_sha256",
                "oracle_sha256", "oracle_review_sha256", "terminal_api_sha256",
                "terminal_review_sha256", "qpc_sha256", "budget_sha256", "phase_go_sha256",
                "reader_probe_sha256", "growth_capsule_sha256", "postexit_finalizer_sha256",
                "clock_handshake_sha256", "history_source_sha256"):
        need(digest(h.get(key)), "history-binding:" + key)
    links = {
        "runner_sha256": "runtime_source", "reader_probe_sha256": "paced_source",
        "growth_capsule_sha256": "profile_source", "postexit_finalizer_sha256": "finalizer_source",
        "clock_handshake_sha256": "clock_source", "budget_sha256": "budget",
        "phase_go_sha256": "phase_go", "qpc_sha256": "guard_source",
        "history_source_sha256": "history_source",
    }
    need(all(h[key] == value[name]["sha256"] for key, name in links.items()), "binding-links")
    need(value["guard_source"]["sha256"] != HISTORICAL_GUARD, "historical-guard-is-not-031")
    guard_schema = control_json(artifact(value["guard_schema"]), 2 * MIB)
    need(guard_schema.get("revision") == "031" and guard_schema.get("source_sha256")
         == value["guard_source"]["sha256"], "explicit-reviewed-guard031-schema")
    catalog = value["case_catalog"]
    closed(catalog, ("source", "ids", "history_case"), "frozen-case-catalog")
    artifact(catalog["source"])
    need(type(catalog["ids"]) is list and len(catalog["ids"]) == 166
         and len(set(catalog["ids"])) == 166
         and catalog["history_case"] == "H1k-H10k-three-cycles-1and4"
         and catalog["history_case"] in catalog["ids"], "complete-Windows166-case-map")
    need(type(value["profile_config"]) is dict, "closed-profile-data-required")
    if execution:
        need(value["runtime_go"] is True and h.get("runtime_go") is True
             and h.get("source_review") is True and h.get("terminal_review") is True
             and h.get("oracle_review") is True and h.get("same_final_pin_native") is True,
             "separate-current-runtime-authority")
        need(time.time() < CUTOFF, "fixed-functional-cutoff")
    return value


def _profiles(bindings):
    closed(bindings, BINDING_KEYS, "profile-binding")
    spec = bindings["profile_source"]
    artifact(spec)
    with _PROFILE_LOCK:
        key = spec["sha256"]
        if key not in _PROFILES:
            loader = importlib.util.spec_from_file_location("owned_profiles_" + key, spec["path"])
            need(loader is not None and loader.loader is not None, "035-profile-loader")
            module = importlib.util.module_from_spec(loader)
            loader.loader.exec_module(module)
            need(Path(module.__file__).resolve() == Path(spec["path"]).resolve(),
                 "035-profile-module-path")
            for name in ("encode_private_record", "fixed_reader_capacity",
                         "derive_history_forecast", "history_phase_profile", "initial_case_profile", "global_trace_profile"):
                need(callable(getattr(module, name, None)), "035-owned-export:" + name)
            artifact(spec)
            _PROFILES[key] = module
        return _PROFILES[key]


def _history(bindings):
    spec = bindings["history_source"]
    artifact(spec)
    need(Path(spec["path"]).name == "history_lane.py", "owned-027-history-validator")
    with _PROFILE_LOCK:
        key = spec["sha256"]
        if key not in _HISTORY:
            loader = importlib.util.spec_from_file_location("owned_history_" + key, spec["path"])
            need(loader is not None and loader.loader is not None, "027-validator-loader")
            module = importlib.util.module_from_spec(loader)
            loader.loader.exec_module(module)
            need(callable(getattr(module, "validate_phase_evidence", None)), "027-validator-export")
            artifact(spec)
            _HISTORY[key] = module
        return _HISTORY[key]


def _encode(value, bindings):
    raw = _profiles(bindings).encode_private_record(value)
    need(type(raw) is bytes and raw.endswith(b"\n"), "035-canonical-record-bytes")
    return raw


def _value_digest(value, bindings):
    return sha(_encode(value, bindings)[:-1])


def _plan(plan):
    need(type(plan) is dict and len(plan.get("phases", [])) == 16
         and plan.get("total_accepted") == 1600 and plan.get("max_offers") == 4000
         and plan.get("max_sealed_losers") == 2400, "frozen1600-4000-2400")
    expected = []
    for donor, initial in (("H1", 1002), ("H10", 10002)):
        row = plan["donors"][donor]
        need(row["root"] == ROOTS[donor] and row["actual_H"] == initial, "exact-existing-donor")
        selector(row["selected"])
        head = initial
        for publishers in (1, 4):
            for cycle in range(4):
                name = donor.lower() + "-p" + str(publishers) + "-" + ("control" if cycle == 0 else "online" + str(cycle))
                expected.append(name)
                phase = next((p for p in plan["phases"] if p["id"] == name), None)
                need(phase is not None and phase["root"] == row["root"]
                     and phase["publishers"] == publishers and phase["H_start"] == head
                     and phase["H_end"] == head + 100 and phase["accepted"] == 100
                     and phase["online"] is (cycle > 0) and phase["slot_interval_ns"] == 400000000,
                     "frozen-phase:" + name)
                head += 100
    need([p["id"] for p in plan["phases"]] == expected, "complete-ordered-phase-map")
    return plan


def reader_capacity(runtime_bindings):
    """Direct owned capacity; 035 owns its source-derived numeric geometry."""
    b = _bindings(runtime_bindings)
    p = _profiles(b)
    result = p.fixed_reader_capacity(
        b["history"]["source_pin"], b["paced_source"]["sha256"],
        b["clock_source"]["sha256"], b["runtime_source"]["sha256"])
    need(type(result) is dict and result.get("warm_horizon_ns") == 45000000000,
         "035-fixed45s-capacity")
    streams = result.get("streams")
    need(type(streams) is list and len(streams) == 4, "four-capacity-streams")
    for number, stream in enumerate(streams):
        need(stream.get("id") == number and stream.get("batches") == 10
             and stream.get("queries_per_batch") == 1000 and stream.get("first_queries") == 10
             and (stream.get("offset_ns"), stream.get("interval_ns"))
             == ((16000000000, 2900000) if number == 0 else (0, 4500000))
             and stream.get("last_native_query_due_ns")
             == stream["offset_ns"] + 9999 * stream["interval_ns"], "fixed-reader-geometry")
    need(result["source_pin"] == b["history"]["source_pin"]
         and result["probe_sha256"] == b["paced_source"]["sha256"]
         and result["clock_handshake_sha256"] == b["clock_source"]["sha256"], "capacity-source-links")
    policy = b["reader_policy"]
    closed(policy, ("publisher_horizon_ns", "reader_deadline_ns", "old_pin_end_bound_ns",
                    "clock_uncertainty_ns", "category_map"), "reader-policy")
    need(integer(policy["publisher_horizon_ns"], 44000000000, 44000000000)
         and integer(policy["reader_deadline_ns"], 48000000000, 48000000000)
         and integer(policy["old_pin_end_bound_ns"], 23600000000, 23600000000)
         and integer(policy["clock_uncertainty_ns"], 1, 1000000), "reader-deadlines")
    need(result["publisher_horizon_ns"] == policy["publisher_horizon_ns"]
         and result["reader_deadline_ns"] == policy["reader_deadline_ns"]
         and result["old_pin_end_bound_ns"] == policy["old_pin_end_bound_ns"]
         and result["category_map"] == policy["category_map"], "exact-owned-profile-policy")
    need(type(policy["category_map"]) is list and len(policy["category_map"]) == 40
         and set(policy["category_map"]) == set(CATEGORIES), "complete-frozen40-category-map")
    need(result.get("exhaustion") == "fail-no-renewal-no-skipped-query"
         and integer(result.get("header_bytes"), 1, 2048)
         and integer(result.get("milestones"), 1, 10), "native-header-exhaustion-bound")
    return copy.deepcopy(result)


def derive_whole_forecast(plan, runtime_bindings, current):
    """No caller formula: directly execute the source-bound 035 calculator."""
    b = _bindings(runtime_bindings)
    _plan(plan)
    need(type(current) is dict, "actual-current-data")
    profiles = _profiles(b)
    model = profiles.derive_history_forecast(copy.deepcopy(plan), copy.deepcopy(b), copy.deepcopy(current))
    need(type(model) is dict and set(FULL_FORECAST_KEYS) <= set(model), "035-whole-model-shape")
    need(model["source_pin"] == b["history"]["source_pin"]
         and model["capsule_sha256"] == b["profile_source"]["sha256"]
         and model["counts"] == plan["finite_command_bounds"], "source-derived-schedule-counts")
    for name in ("F05_bytes", "F05_files", "protocol_bytes", "protocol_files", "evidence_bytes", "evidence_files"):
        need(integer(model[name]), "whole-model:" + name)
    for name in ("expanded_trace_sha256", "all_sink_membership_sha256", "profile_sha256"):
        need(digest(model[name]), "whole-model-proof:" + name)
    need(model["F05_bytes"] <= 1073741824 and model["F05_files"] <= 400000,
         "whole-F05-admission")
    artifact(b["profile_source"])
    return copy.deepcopy(model)


def _observation(value, b, root, action=None):
    fields = ("token", "input", "env", "exit", "violations", "binary_sha256",
              "source_pin", "identity", "start_ns", "end_ns", "spawn_ns", "value",
              "raw_ref", "meta_ref", "case_id", "scope_id", "case_scope",
              "stdout_eof", "stderr_eof", "process_reaped", "io_closed",
              "native_closed", "callbacks_joined", "peak_rss", "input_complete", "start_observed")
    closed(value, fields, "owned-call-observation")
    need(digest(value["token"], 32) and value["input"].get("root") == str(root)
         and (action is None or value["input"].get("action") == action), "exact-call-input")
    need(value["source_pin"] == b["history"]["source_pin"]
         and value["binary_sha256"] == b["history"]["binary_sha256"]
         and value["violations"] == [], "uncensored-same-producer")
    for name in ("stdout_eof", "stderr_eof", "process_reaped", "io_closed", "native_closed", "callbacks_joined", "input_complete", "start_observed"):
        need(value[name] is True, "positive-owned-closure:" + name)
    need(integer(value["start_ns"], 1) and integer(value["end_ns"], value["start_ns"])
         and integer(value["spawn_ns"], 0) and type(value["value"]) is dict, "actual-call-clocks")
    closed(value["identity"], ("pid", "creation_id", "image_sha256", "contained"), "native-identity")
    identity = value["identity"]
    need(integer(identity["pid"], 1, (1 << 32) - 1) and type(identity["creation_id"]) is str
         and 1 <= len(identity["creation_id"]) <= 128 and identity["contained"] is True
         and identity["image_sha256"] == b["history"]["binary_sha256"], "actual-owned-image-identity")
    reference(value["meta_ref"])
    need(type(value["case_id"]) is str and type(value["scope_id"]) is str
         and digest(value["case_scope"], 32), "actual-case-scope-metadata")
    raw = reference(value["raw_ref"])
    need(raw.get("schema") == CAPTURE and raw["call"] == value["token"], "single-owned-raw-capture")
    limit = 256 * MIB if value["input"]["action"] in ("resolve", "point", "inspect", "lookup-bench") else 2048 * MIB
    need(integer(value["peak_rss"], 1, limit), "native-RSS-censor")
    return value


def _response(value, status="ok"):
    need(type(value) is dict and value.get("status") == status
         and "result" in value and "error" in value and type(value.get("metrics")) is dict
         and integer(value.get("wall_ns")), "actual-engine-envelope")
    return value["result"]


def _probe(value, request, count, expected_digest):
    closed(value, ("category", "id", "first_lookup_ns", "warm_ns", "warm_median_ns",
                   "warm_p95_ns", "warm_min_ns", "warm_max_ns", "result_digest",
                   "opens", "pack_storage_bytes", "decoded_record_bytes", "index_cache_hits"),
           "native-probe-row")
    need(value["category"] == request["category"] and value["id"] == request["id"]
         and value["result_digest"] == expected_digest and integer(value["first_lookup_ns"])
         and type(value["warm_ns"]) is list and len(value["warm_ns"]) == count
         and all(integer(n) for n in value["warm_ns"]), "exact-first-warm-id-result")
    return value


class HistoryRunner:
    api = "history-native-v1"

    def __init__(self, host, bindings):
        self.host, self.b = host, copy.deepcopy(_bindings(bindings))
        self.h, self.ports = None, None
        self.plan, self.current, self.model = None, None, None
        self.session = None
        self.closed_phases = {}
        self.completed_phases = {}
        self.case_indices = {}
        self.trace_ref = None

    def _activate(self):
        host = self.host
        auth = host.history_authorization
        need(type(auth) is dict and auth.get("go") is True
             and auth.get("final_pin") == self.b["history"]["source_pin"]
             and digest(auth.get("receipt_sha256")), "positive-host-authorization-before-properties")
        self.h = host.harness
        need(self.h.final_pin == self.b["history"]["source_pin"]
             and self.h.driver.binary_sha256 == self.b["history"]["binary_sha256"], "same-Harness-image")
        need(sha(Path(inspect.getsourcefile(type(self.h))).read_bytes()) == self.b["history"]["core_sha256"], "exact-accepted-Harness-source")
        self.ports = getattr(host, "history_ports", None)
        need(self.ports is not None and all(callable(getattr(self.ports, n, None)) for n in PORT_METHODS),
             "concrete-owned-native-ports-required")
        actual_file = inspect.getsourcefile(type(self.ports))
        need(actual_file and Path(actual_file).resolve() == Path(self.b["ports_source"]["path"]).resolve(),
             "native-ports-defined-in-reviewed-source")
        self._check_ports()

    def source_bindings(self):
        return copy.deepcopy(_bindings(self.b))

    def bindings(self):
        return copy.deepcopy(self.b["history"])

    def _check_ports(self):
        result = self.ports.bindings()
        expected = {
            "api": PORT_API, "platform": "Windows", "source_pin": self.b["history"]["source_pin"],
            "source_sha256": self.b["ports_source"]["sha256"],
            "review_sha256": self.b["ports_review"]["sha256"],
            "guard_sha256": self.b["guard_source"]["sha256"],
            "guard_schema_sha256": self.b["guard_schema"]["sha256"],
            "clock_sha256": self.b["clock_source"]["sha256"],
            "paced_sha256": self.b["paced_source"]["sha256"],
            "capture_schema": CAPTURE, "native_meta_schema": "native-call-meta-v2",
            "case_observations_scope": "case-open-through-close",
        }
        need(result == expected, "native-port-exact-closure")
        return result

    def _execute(self):
        need(self.h is not None and self.ports is not None, "authorized-preflight-required")
        _bindings(self.b, execution=True)
        self._check_ports()
        need(self.host.final_pin == self.b["history"]["source_pin"], "host-final-pin")
        poll = self.host.poll()
        need(type(poll) is dict and poll.get("live") is True and digest(poll.get("receipt_sha256")),
             "positive-native-fast-poll")

    def preflight(self, plan, forecast, release):
        self._activate()
        self._execute()
        need(release == self.bindings(), "exact-history-release")
        _plan(plan)
        current = copy.deepcopy(self.b["profile_config"]["components"]["current"])
        model = derive_whole_forecast(plan, self.b, current)
        need(model["F05_bytes"] == forecast["bytes"] and model["F05_files"] == forecast["files"]
             and model["protocol_bytes"] == forecast["protocol"]["additional_bytes"]
             and model["evidence_bytes"] == forecast["evidence"]["additional_bytes"],
             "same-source-derived-whole-reserves")
        proof = self.ports.build_binding()
        need(proof.get("source_pin") == release["source_pin"]
             and proof.get("binary_sha256") == release["binary_sha256"]
             and proof.get("engine_manifest_sha256") == release["engine_manifest_sha256"]
             and digest(proof.get("receipt_sha256")), "actual-reviewed-build")
        need(proof.get("paced_schema") == self.b["paced_schema"]
             and set(ROOTS.values()) <= set(proof.get("paced_roots", [])), "compiled-F05-paced-ABI")
        self.ports.verify_case_catalog(copy.deepcopy(self.b["case_catalog"]))
        checkpoint = self.host.checkpoint("history-whole-preflight", self.b["profile_config"]["components"])
        need(checkpoint.get("go") is True and checkpoint.get("no_other_semantic_writer") is True,
             "fresh-exclusive-whole-admission")
        for phase in plan["phases"]:
            data = _profiles(self.b).history_phase_profile(plan, phase, self.b, current)
            need(type(data) is dict and data.get("complete") is True and data.get("source_pin") == self.h.final_pin, "all-profiles-before-write")
            closed(data["metric_names"], (*COUNTERS, "forbidden_global_operations", "unchanged_historical_body_opens"), "source-counter-map")
            need(all(n in proof["metric_fields"] for n in data["metric_names"].values()), "compiled-telemetry-schema")
            need(data.get("case_index_scope") == "closed-case-all-engine-and-checker-calls"
                 and integer(data.get("case_index_max_calls"), phase["max_offers"] + 40),
                 "source-derived-complete-case-index-geometry")
            for k in ("origin_record_bytes", "slot_record_bytes", "call_index_record_bytes", "closure_record_bytes"):
                need(integer(data[k], 1, MAX_PHASE), "source-record-width:"+k)
        for case in self._initial_cases(plan):
            data = _profiles(self.b).initial_case_profile(plan, case["id"], self.b, current)
            need(data.get("complete") is True and data.get("source_pin") == self.h.final_pin
                 and integer(data.get("case_index_max_calls"), 1)
                 and integer(data.get("call_index_record_bytes"), 1, MAX_PHASE),
                 "source-derived-initial-case-index")
        trace = _profiles(self.b).global_trace_profile(plan, self.b, current)
        need(trace.get("complete") is True and trace.get("case_count") == 22
             and integer(trace.get("trace_record_bytes"), 1, MAX_PHASE), "source-derived-global-trace")
        _history(self.b)  # Reviewed direct validator, never a caller callback.
        self.plan, self.current, self.model = copy.deepcopy(plan), current, model
        capacity = reader_capacity(self.b)
        return {
            "go": True, "plan_sha256": plan["plan_sha256"], "fresh_whole_census": True,
            "no_other_semantic_writer": True, "no_clone": True,
            "F05_bytes": 1073741824, "F05_files": 400000,
            "reader_capacity_sha256": _value_digest(capacity, self.b),
            "quantitative_profile_sha256": model["profile_sha256"],
            "checkpoint_ref": reference(checkpoint["raw_ref"]),
        }

    def adopt(self, h, donor):
        self._execute()
        need(h is self.h and donor["root"] == ROOTS[donor["id"]], "exact-owned-donor")
        need(h.selected(donor["root"]) == donor["selected"], "donor-selected-not-mutated")
        authority = self.ports.donor_authority(copy.deepcopy(donor))
        closed(authority, ("records", "accepted_inputs", "request_provenance", "seed", "chain_refs"),
               "durable-donor-authority")
        need(type(authority["records"]) is list and len(authority["records"]) == donor["actual_H"],
             "complete-durable-accepted-recipe")
        for key, field in (("accepted_inputs", "accepted_inputs_sha256"),
                           ("request_provenance", "request_provenance_sha256"),
                           ("seed", "seed_receipt_sha256")):
            ref = reference(authority[key])
            need(ref["sha256"] == donor[field], "actual-authority:" + key)
            raw = self.ports.read_reference(ref)
            need(type(raw) is bytes and len(raw) == ref["bytes"] and sha(raw) == ref["sha256"],
                 "authority-chain-bytes")
        need(type(authority["chain_refs"]) is list and authority["chain_refs"], "noninventory-provenance-chain")
        for item in authority["chain_refs"]:
            ref = reference(item)
            raw = self.ports.read_reference(ref)
            need(sha(raw) == ref["sha256"] and len(raw) == ref["bytes"], "complete-provenance-chain")
        h.adopt_inputs(donor["root"], authority["records"],
                       seed_receipt_sha256=donor["seed_receipt_sha256"])
        return {"complete": True, "actual_H": len(h.ledgers[donor["root"]].accepted),
                "accepted_inputs_sha256": donor["accepted_inputs_sha256"],
                "request_provenance_sha256": donor["request_provenance_sha256"]}

    def _last(self, root, action):
        return copy.deepcopy(_observation(self.ports.current_call_observation(), self.b, root, action))

    def lookup(self, h, donor, lookup_plan):
        self._execute()
        need(h is self.h and h.selected(donor["root"]) == donor["selected"], "lookup-fixed-selected")
        root = donor["root"]
        representations = []
        for fmt in ("legacy", "paged"):
            if fmt == "paged":
                h.call(root, "packing", operation="convert", codec="zstd",
                       reserve_bytes=320 * MIB, cleanup=False)
            fresh, warm = [], []
            for request in donor["lookup_requests"]:
                expected = donor["expected_outcomes"][request["category"]]
                for sample in range(50):
                    value = h.call(root, "resolve", request_id=request["id"])
                    need(_response(value) == expected, "fresh-exact-original-outcome")
                    row = self._last(root, "resolve")
                    fresh.append({"category": request["category"], "id": request["id"], "sample": sample,
                                  "result": expected, "engine_ns": value["wall_ns"],
                                  "spawn_ns": row["spawn_ns"], "pid": row["identity"]["pid"],
                                  "rss_bytes": row["peak_rss"], "metrics": value["metrics"],
                                  "raw_sha256": row["raw_ref"]["sha256"]})
                value = h.call(root, "lookup-bench", count=500, requests=[request])
                result = _response(value)
                need(type(result.get("rows")) is list and len(result["rows"]) == 1, "one-original-warm-category")
                probe = _probe(result["rows"][0], request, 500, _value_digest(expected, self.b))
                row = self._last(root, "lookup-bench")
                warm.append({"category": request["category"], "id": request["id"], "result": expected,
                             "first_ns": probe["first_lookup_ns"], "warm_ns": probe["warm_ns"],
                             "metrics": value["metrics"], "raw_sha256": row["raw_ref"]["sha256"]})
            need(h.selected(root) == donor["selected"], "lookup-no-semantic-write")
            oracle = h.verify(root, "lookup-" + donor["id"] + "-" + fmt + "-internal-full")
            need(oracle.get("actual_H") == donor["actual_H"]
                 and oracle.get("final_pin") == h.final_pin and digest(oracle.get("receipt_sha256"))
                 and h.selected(root) == donor["selected"], "actual-format-complete-oracle")
            representations.append({"format": fmt, "source_pin": h.final_pin,
                "binary_sha256": h.driver.binary_sha256, "actual_H": donor["actual_H"],
                "full_oracle_sha256": oracle["receipt_sha256"], "fresh": fresh, "warm": warm})
        # Two real per-format oracles per donor; complete schedule has26 full oracles.
        h.cleanup(root, max_batches=160)
        enabled = _response(h.call(root, "lifecycle", operation="enable-terminal-v2", explicit_disposable_opt_in=True))
        need(enabled.get("status") in ("terminal-v2-enabled", "terminal-v2-already-enabled"), "v2-optin-completed")
        quota = h.call(root, "lifecycle", operation="quota")
        need(_response(quota).get("registry_version") == 2, "actual-v2-optin")
        ready = {"actual_cleanup_complete": True, "private_v2_enabled": True,
                 "NOOP_preconditions": h.selected(root) == donor["selected"],
                 "initial_active_reservations": len(_response(quota).get("reservations", {})),
                 "no_semantic_change": h.selected(root) == donor["selected"],
                 "receipt_sha256": self._last(root, "lifecycle")["raw_ref"]["sha256"]}
        need(ready["initial_active_reservations"] == 0, "initial-v2-active-empty")
        raw = self.host.record("history-initial-lookup", {
            "representations": [{"format": r["format"],
                "fresh": [{k: v[k] for k in ("category", "id", "sample", "raw_sha256")} for v in r["fresh"]],
                "warm": [{k: v[k] for k in ("category", "id", "raw_sha256")} for v in r["warm"]]}
                for r in representations], "oracle_refs": [r["full_oracle_sha256"] for r in representations], "ready": ready},
                               category="evidence", max_bytes=MAX_PHASE)
        return {"donor": donor["id"], "actual_H": donor["actual_H"],
                "selected_before": donor["selected"], "selected_after": h.selected(root),
                "source_pin": h.final_pin, "binary_sha256": h.driver.binary_sha256,
                "order": ["final-candidate-legacy", "convert-paged", "final-candidate-paged"],
                "semantic_writes": 0, "representations": representations, "complete_oracle": True,
                "semantic_unchanged": True, "raw_samples_retained": True,
                "receipt_sha256": raw["sha256"], "post_measurement_ready": ready}

    def release_definitive_conflicts(self, h, root, requests, outcomes, candidates):
        self._execute()
        need(h is self.h and len(requests) == len(outcomes) == 4
             and type(candidates) is dict, "four-actual-sealed-owner-observations")
        receipts = []
        for request, result in zip(requests, outcomes):
            if result.get("status") == "ok":
                continue
            need(result.get("error") == "conflict:expected-publication", "definitive-conflict-only")
            owner = candidates[request["request"]]
            candidate = control_json(owner["bytes"], 4 * MIB)
            need(candidate.get("request") == request and candidate.get("expected") == request["base"]
                 and owner["path"] == "attempts/" + candidate["attempt"] + "/candidate.json",
                 "exact-sealed-candidate")
            value = h.call(root, "lifecycle", operation="release-definitive-conflict",
                           attempt=candidate["attempt"])
            actual = _response(value)
            row = self._last(root, "lifecycle")
            need(actual.get("status") in ("terminal-charged", "terminal-already-charged")
                 and actual.get("attempt") == candidate["attempt"] and digest(actual.get("receipt_sha256"))
                 and actual.get("reclaimed_bytes") == 0, "exact-CP4-terminal-reply")
            path = str(Path(root)/"abandoned"/candidate["attempt"])
            raw = self.ports.read_owned(path, 8192)
            need(type(raw) is bytes and sha(raw) == actual["receipt_sha256"], "immutable-terminal-fence")
            terminal = control_json(raw, 8192)
            need(terminal.get("candidate_digest") == sha(owner["bytes"])
                 and terminal.get("request") == request["request"] and terminal.get("expected") == request["base"]
                 and terminal.get("reason") == "definitive-expected-publication-conflict"
                 and terminal.get("charge_bytes") == actual.get("charge_bytes"), "terminal-owned-binding")
            ref = {"path": path, "bytes": len(raw), "sha256": sha(raw)}
            receipts.append({"request": request["request"], "attempt": candidate["attempt"],
                             "receipt_sha256": ref["sha256"], "receipt_bytes": len(raw),
                             "retained_charge": actual["charge_bytes"],
                             "selection_never_started": True, "stale_base_authenticated": True,
                             "reclaimed_bytes": 0, "raw_call_ref": row["raw_ref"]})
        need(len(receipts) == 3, "exact-three-terminal-losers")
        if self.session is not None:
            self.session.terminals.extend(copy.deepcopy(receipts))
        normalized = [{k: v for k, v in row.items() if k != "raw_call_ref"} for row in receipts]
        registry = _response(h.call(root, "lifecycle", operation="quota"))
        need(registry.get("registry_version") == 2 and type(registry.get("reservations")) is dict, "actual-v2-registry")
        return {"api_sha256": self.b["history"]["terminal_api_sha256"],
                "receipts": normalized, "retained_raw": True, "reclaimed_bytes": 0,
                "semantic_collect_called": False, "global_history_scan": False, "active_slots_after": len(registry["reservations"])}

    def phase(self, h, phase):
        need(h is self.h and self.plan is not None and self.session is None, "one-admitted-phase")
        need(any(p == phase for p in self.plan["phases"]), "exact-planned-phase")
        return Phase(self, phase)

    def _initial_cases(self, plan=None):
        plan = self.plan if plan is None else plan
        need(plan is not None, "preflight-before-case-indices")
        return [{"id": label, "root": d["root"]} for identity, d in plan["donors"].items()
                for label in ["lookup-" + identity, "bootstrap-" + identity + "-p1",
                              "bootstrap-" + identity + "-p4"]]

    def seal_case(self, case_id):
        self._execute()
        phase = next((c for c in self._initial_cases() if c["id"] == case_id), None)
        need(phase is not None and case_id not in self.closed_phases, "one-authenticated-initial-case")
        context = self.ports.seal_context(case_id)
        need(context.get("case_closed") is True
             and context.get("case_id") == self.b["case_catalog"]["history_case"]
             and context.get("scope_id") == case_id and digest(context.get("case_scope"), 32)
             and integer(context.get("case_start_ns"), 1)
             and integer(context.get("case_end_ns"), context["case_start_ns"], time.perf_counter_ns()),
             "actual-completed-initial-case")
        for key, kind in (("owned_drain_ref", "owned-drain"), ("checkpoint_ref", "checkpoint"),
                          ("case_closure_ref", "case-close")):
            _case_attachment(self.ports, context[key], context, self.b, phase, kind)
        self.closed_phases[case_id] = {"state": "sealing-initial"}
        ref = _case_call_index(self, phase, context)
        self.closed_phases[case_id] = {"index": ref}
        return ref

    def seal_trace(self):
        self._execute()
        need(self.trace_ref is None, "one-complete-source-case-trace")
        expected = [p["id"] for p in self.plan["phases"]] + [c["id"] for c in self._initial_cases()]
        need(len(expected) == 22 and set(self.case_indices) == set(expected)
             and all("body" in self.closed_phases.get(p["id"], {}) for p in self.plan["phases"]),
             "all22-source-case-indices-and-sixteen-sealed-bodies")
        body = {"api": "history-expanded-case-index-v2", "source_pin": self.b["history"]["source_pin"],
                "profile_sha256": self.b["profile_source"]["sha256"],
                "case_indices": [{"id": name, "index_ref": self.case_indices[name]} for name in expected]}
        profile = _profiles(self.b).global_trace_profile(self.plan, self.b, self.current)
        raw = _encode(body, self.b)
        need(profile.get("complete") is True and profile.get("case_count") == 22
             and len(raw) <= profile["trace_record_bytes"], "035-complete-trace-width")
        ref = reference(self.host.record("history-expanded-case-index", raw, category="evidence",
                        max_bytes=profile["trace_record_bytes"]), MAX_PHASE)
        need(ref["bytes"] == len(raw) and ref["sha256"] == sha(raw)
             and _read_record(self.ports, ref) == body, "actual-completed-global-trace")
        self.trace_ref = ref
        return copy.deepcopy(ref)

    def seal_final_provisional(self, phase, evidence):
        return seal_final_provisional(self.host, phase, evidence, self.b)


def make_runner(host, runtime_bindings):
    return HistoryRunner(host, runtime_bindings)


class Phase:
    """Four real finite streams; no reader renewal or final-Job authority."""
    def __init__(self, runner, phase):
        self.r, self.host, self.h = runner, runner.host, runner.h
        self.b, self.phase = runner.b, copy.deepcopy(phase)
        self.a, self.ports = self.h.a, runner.ports
        self.capacity = reader_capacity(self.b)
        self.profile = None
        self.handles, self.readers, self.slots, self.terminals = [], [], [], []
        self.started, self.markers, self.failures = {}, {}, []
        self.lock = threading.RLock()
        self.origin, self.namespace, self.anchor_ref = None, None, None
        self.finished, self.closed, self.publishers_done = False, False, False
        self.physical_states = []
        self.start_ns, self.end_ns = 0, 0
        self._finish_value = None

    def __enter__(self):
        self.r._execute()
        self.r.session = self
        try:
            self.start()
            return self
        except BaseException as primary:
            self.close(primary)
            raise

    def __exit__(self, kind, error, traceback):
        self.close(error)
        return False

    def _poll(self):
        poll = self.host.poll()
        need(type(poll) is dict and poll.get("live") is True and digest(poll.get("receipt_sha256")), "positive-fast-poll")
        need(time.time() < CUTOFF, "fixed-functional-cutoff")
        with self.lock:
            need(not self.failures, "paced-reader-failure:" + repr(self.failures))
        if self.origin is not None:
            now = time.perf_counter_ns()
            need(now < self.origin["origin_perf_ns"] + self.capacity["reader_deadline_ns"],
                 "fixed-reader-deadline")
            if not self.publishers_done:
                need(now < self.origin["origin_perf_ns"] + self.capacity["publisher_horizon_ns"],
                     "finite-read-load-cannot-cover-publishers")

    def _started(self, reader, batch):
        def callback(event):
            proof = self.ports.observe_started(self.phase["id"], reader, batch, copy.deepcopy(event))
            closed(proof, ("token", "pid", "creation_id", "image_sha256", "rss", "contained", "raw_ref"),
                   "positive-reader-start")
            need(digest(proof["token"], 32) and integer(proof["pid"], 1, (1 << 32) - 1)
                 and proof["contained"] is True and integer(proof["rss"], 1, 256 * MIB)
                 and proof["image_sha256"] == self.b["history"]["binary_sha256"], "actual-reader-start")
            reference(proof["raw_ref"])
            with self.lock:
                need((reader, batch) not in self.started, "reader-start-once")
                self.started[(reader, batch)] = proof
        return callback

    def _request(self, reader, batch):
        category = self.capacity["category_map"][reader * 10 + batch]
        donor = self.r.plan["donors"][self.phase["donor"]]
        request = next(row for row in donor["lookup_requests"] if row["category"] == category)
        return copy.deepcopy(request), donor["expected_outcomes"][category]

    def _pace_fields(self, reader, batch):
        return {
            "version": 1, "family": "f05-selected", "phase_id": self.phase["id"], "reader": reader, "batch": batch,
            "control_dir": self.namespace["path"],
            "anchor_sha256": None if batch == 0 else self.anchor_ref["sha256"],
            "ready_sha256": None if batch == 0 else self.markers[reader]["ref"]["sha256"],
        }

    def _read_stream(self, reader):
        try:
            for batch in range(10):
                self._poll()
                request, expected = self._request(reader, batch)
                value = self.h.call(
                    self.phase["root"], "lookup-bench", count=1000, requests=[request],
                    concurrent_pacing=self._pace_fields(reader, batch),
                    env={"FILE_NATIVE_PACED_ROOT": self.namespace["canonical_root"]},
                    on_started=self._started(reader, batch), timeout=50)
                row = self.r._last(self.phase["root"], "lookup-bench")
                result = _response(value)
                need(type(result.get("rows")) is list and len(result["rows"]) == 1, "one-paced-row")
                probe = _probe(result["rows"][0], request, 1000, _value_digest(expected, self.b))
                header = result.get("concurrent_pacing")
                self._header(reader, batch, header, row)
                normalized = {
                    "reader": reader, "batch": batch, "request": request, "header": copy.deepcopy(header),
                    "start_ns": row["start_ns"], "end_ns": row["end_ns"],
                    "identity": row["identity"], "peak_rss": row["peak_rss"],
                    "first_ns": probe["first_lookup_ns"], "raw_ref": row["raw_ref"], "meta_ref": row["meta_ref"],
                }
                with self.lock:
                    self.readers.append(normalized)
                del value, row, result, probe
            return {"reader": reader, "calls": 10}
        except BaseException as error:
            with self.lock:
                self.failures.append({"reader": reader, "error": repr(error)})
            raise

    def start(self):
        self.start_ns = time.perf_counter_ns()
        self.a.begin_phase(self.phase["root"], self.phase["id"])
        self.physical_states.append((self.start_ns, self.h.state(self.phase["root"])))
        self.profile = _profiles(self.b).history_phase_profile(
            copy.deepcopy(self.r.plan), copy.deepcopy(self.phase), copy.deepcopy(self.b),
            copy.deepcopy(self.r.current))
        need(type(self.profile) is dict and self.profile.get("complete") is True
             and self.profile.get("source_pin") == self.h.final_pin
             and digest(self.profile.get("profile_sha256")), "035-concrete-phase-profile")
        self.namespace = self.ports.authorize_namespace(copy.deepcopy(self.phase), copy.deepcopy(self.profile))
        closed(self.namespace, ("path", "canonical_root", "category", "receipt_ref"), "pace-namespace")
        reference(self.namespace["receipt_ref"])
        need(Path(self.namespace["path"]).is_absolute(), "absolute-phase-namespace")
        selected = copy.deepcopy(self.h.selected(self.phase["root"]))
        self.ports.enter_phase(copy.deepcopy(self.phase), copy.deepcopy(self.profile),
                               copy.deepcopy(self.capacity))
        for reader in range(4):
            self.handles.append(self.a.spawn(lambda reader=reader: self._read_stream(reader)))
        end = time.perf_counter_ns() + 4_000_000_000
        while len(self.markers) < 4:
            self._poll()
            need(time.perf_counter_ns() < end, "all-ready-before-native5s-anchor-timeout")
            for reader in range(4):
                if reader in self.markers:
                    continue
                with self.lock:
                    start = copy.deepcopy(self.started.get((reader, 0)))
                if start is None:
                    continue
                path = str(Path(self.namespace["path"]) / ("reader-" + str(reader) + ".ready.json"))
                raw = self.ports.read_owned(path, MAX_CONTROL)
                if raw is None:
                    continue
                marker = control_json(raw)
                self._ready(reader, marker, start, selected)
                proof = self.ports.verify_ready(marker, start, self.namespace)
                need(proof.get("same_opened_native_identity") is True and digest(proof.get("receipt_sha256")),
                     "ready-native-handle-identity")
                self.markers[reader] = {"value": marker, "ref": {
                    "path": path, "offset": 0, "bytes": len(raw), "sha256": sha(raw)}}
            if len(self.markers) < 4:
                time.sleep(.005)
        physical = {self.markers[i]["value"]["physical"]["id"] for i in range(4)}
        need(len(physical) == 1 and len({self.started[(i, 0)]["pid"] for i in range(4)}) == 4,
             "four-real-distinct-same-generation-readers")
        clock = self.ports.clock_sample()
        closed(clock, ("kind", "domain", "frequency", "ticks", "perf_before_ns", "perf_after_ns",
                       "receipt_ref"), "native-QPC-perf-bracket")
        need(clock["kind"] == "QueryPerformanceCounter" and integer(clock["frequency"], 1)
             and integer(clock["ticks"], 1) and integer(clock["perf_before_ns"], 1)
             and integer(clock["perf_after_ns"], clock["perf_before_ns"])
             and clock["perf_after_ns"] - clock["perf_before_ns"]
             <= self.b["reader_policy"]["clock_uncertainty_ns"], "source-bound-clock-proof")
        reference(clock["receipt_ref"])
        origin = clock["ticks"] + clock["frequency"]
        need(all(self.markers[i]["value"]["ready_qpc"] < origin
                 and self.markers[i]["value"]["frequency"] == clock["frequency"] for i in range(4)),
             "all-native-ready-before-one-future-origin")
        anchor = {"clock_kind": "windows-qpc", "version": 1, "phase_id": self.phase["id"],
                  "frequency": clock["frequency"], "origin": origin}
        raw = _encode(anchor, self.b)
        need(len(raw) <= MAX_CONTROL, "anchor-size")
        self.anchor_ref = reference(self.ports.write_once(
            str(Path(self.namespace["path"]) / "anchor.json"), raw, self.namespace["category"]))
        need(self.anchor_ref["sha256"] == sha(raw), "actual-anchor-bytes")
        origin_perf = (clock["perf_before_ns"] + clock["perf_after_ns"]) // 2 + 1_000_000_000
        marks = []
        for reader in range(4):
            m = self.markers[reader]
            marks.append({
                "pid": m["value"]["pid"], "creation_id": self.started[(reader, 0)]["creation_id"],
                "ready_qpc": m["value"]["ready_qpc"], "selected": selected,
                "physical_generation": m["value"]["physical"]["id"],
                "receipt_sha256": m["ref"]["sha256"],
            })
        body = {"phase": self.phase["id"], "origin_perf_ns": origin_perf,
                "origin_qpc": origin, "qpc_frequency": clock["frequency"],
                "clock_handshake_sha256": self.b["clock_source"]["sha256"], "markers": marks}
        ref = self.host.record("history-reader-origin", _encode({
            "origin": body, "anchor_ref": self.anchor_ref, "clock": clock,
            "uncertainty_ns": clock["perf_after_ns"] - clock["perf_before_ns"]}, self.b),
            category="evidence", max_bytes=self.profile["origin_record_bytes"])
        body["receipt_sha256"] = reference(ref)["sha256"]
        self.origin = body
        self.ports.set_origin(self.phase["id"], copy.deepcopy(body), copy.deepcopy(self.capacity))
        need(time.perf_counter_ns() < origin_perf, "future-origin-missed-no-reset")

    def _ready(self, reader, marker, started, selected):
        required = ("clock_kind", "family", "version", "phase_id", "reader", "batch", "pid", "request", "category",
                    "selector", "physical", "catalog", "capture_qpc", "first_done_qpc", "ready_qpc", "frequency")
        closed(marker, required, "native-ready-marker")
        request, _ = self._request(reader, 0)
        need(marker["clock_kind"] == "windows-qpc" and marker["family"] == "f05-selected"
             and marker["version"] == 1 and marker["phase_id"] == self.phase["id"]
             and marker["reader"] == reader and marker["batch"] == 0
             and marker["pid"] == started["pid"] and marker["request"] == request["id"]
             and marker["category"] == request["category"] and marker["selector"] == selected,
             "actual-ready-capture-request")
        closed(marker["physical"], ("id", "ordinal", "plan"), "ready-physical-tuple")
        initial = self.physical_states[0][1]["store"]
        need(marker["physical"] == initial["physical"] and marker["catalog"] == initial["catalog"],
             "ready-complete-physical-capture")
        need(integer(marker["capture_qpc"], 1)
             and integer(marker["first_done_qpc"], marker["capture_qpc"])
             and integer(marker["ready_qpc"], marker["first_done_qpc"])
             and integer(marker["frequency"], 1) and digest(marker["catalog"]), "ready-native-clock")

    def _header(self, reader, batch, header, row):
        need(type(header) is dict and len(_encode(header, self.b)) - 1 <= 2048, "native-paced-header")
        need(header.get("clock_kind") == "windows-qpc" and header.get("family") == "f05-selected"
             and header.get("version") == 1 and header.get("phase_id") == self.phase["id"]
             and header.get("reader") == reader and header.get("batch") == batch
             and header.get("pid") == row["identity"]["pid"] and header.get("completed") == 1000
             and header.get("origin") == self.origin["origin_qpc"]
             and header.get("frequency") == self.origin["qpc_frequency"]
             and header.get("anchor_sha256") == self.anchor_ref["sha256"]
             and header.get("ready_sha256") == self.markers[reader]["ref"]["sha256"],
             "actual-native-paced-identity")
        closed(header.get("physical"), ("id", "ordinal", "plan"), "header-physical-tuple")
        need(integer(header["physical"]["ordinal"]) and digest(header["physical"]["plan"])
             and type(header["physical"]["id"]) is str and digest(header.get("catalog")),
             "native-full-physical-binding")
        if batch == 0:
            marker = self.markers[reader]["value"]
            need(all(header.get(k) == marker[k] for k in ("physical", "catalog", "selector")),
                 "first-header-matches-actual-ready-capture")
        need(header.get("period_us") == (2900 if reader == 0 else 4500)
             and header.get("hold_us") == (16000000 if reader == 0 else 0)
             and integer(header.get("max_lag_ticks"), 0, header["frequency"] // 10),
             "exact-native-cadence-and-max100ms-lag")
        need(header.get("deadline_ok") is True and header.get("cache_after_drop_bytes") == 0
             and integer(header.get("cache_peak_bytes"), 0, 64 * MIB)
             and integer(header.get("successors"), 0, 60), "native-deadline-pins-cache")
        marks = header.get("milestones")
        need(type(marks) is list and len(marks) == 10, "all-ten-native-milestones")
        for index, mark in enumerate(marks):
            need(type(mark) is list and len(mark) == 3 and mark[0] == batch * 1000 + index * 111
                 and integer(mark[1], 1) and integer(mark[2], mark[1]), "actual-query-milestone")
        need(integer(header.get("io_done"), marks[-1][2])
             and integer(header.get("pins_dropped"), header["io_done"])
             and integer(header.get("reply_ready"), header["pins_dropped"]), "actual-IOfinish-pinsdrop")
        if reader == 0 and batch == 0:
            end = self._perf(header["pins_dropped"])
            need(end <= self.origin["origin_perf_ns"] + self.capacity["old_pin_end_bound_ns"],
                 "old-first-batch-pins-ended-before-generation-cap")

    def _perf(self, ticks):
        return self.origin["origin_perf_ns"] + (
            (ticks - self.origin["origin_qpc"]) * 1_000_000_000 // self.origin["qpc_frequency"])

    def tick(self, progress):
        self._poll()
        need(type(progress) is dict and progress.get("stage") in
             ("reader-origin-before-publish", "before-offer", "after-offer"), "closed-progress-stage")
        if progress["stage"] == "reader-origin-before-publish":
            return copy.deepcopy(self.origin)
        number = progress["number"]
        need(integer(number, 0, 99), "logical-slot-number")
        offered = self.origin["origin_perf_ns"] + number * 400000000
        if progress["stage"] == "before-offer":
            need(number == len(self.slots) and progress["offered_ns"] == offered, "fixed-offered-clock")
            return {"offered_ns": offered}
        need(number == len(self.slots) and progress["offered_ns"] == offered, "actual-after-offer-order")
        rows = [_observation(row, self.b, self.phase["root"])
                for row in self.ports.phase_call_observations(self.phase["id"])]
        requests = progress["result"]["requests"]
        names = [q["request"] for q in requests]
        offers = [r for r in rows if r["input"].get("action") == "publish"
                  and r["input"].get("request", {}).get("request") in names]
        need(len(offers) == self.phase["publishers"] and len({r["token"] for r in offers}) == len(offers),
             "all-actual-native-offer-rows")
        normalized = []
        for request in requests:
            row = next(r for r in offers if r["input"]["request"] == request)
            value = row["value"]
            accepted = value.get("status") == "ok"
            outcome = value.get("result") or {}
            need(row["exit"] == (0 if accepted else 2), "exact-native-offer-exit")
            need(accepted or value.get("error") == "conflict:expected-publication",
                 "only-exact-sealed-stale-not-censor")
            normalized.append({
                "request": copy.deepcopy(request),
                "outcome": "accepted" if accepted else "definitive-stale-sealed",
                "result": outcome.get("result") if accepted else None,
                "selected": outcome.get("selected") if accepted else None,
                "engine_ns": value["wall_ns"], "spawn_ns": row["spawn_ns"],
                "start_ns": row["start_ns"], "end_ns": row["end_ns"],
                "raw_sha256": row["raw_ref"]["sha256"], "error": value.get("error"),
            })
        if self.phase["online"]:
            need(type(progress.get("state")) is dict, "actual-online-state")
            self.physical_states.append((time.perf_counter_ns(), copy.deepcopy(progress["state"])))
        done = time.perf_counter_ns()
        dispatch = min(row["start_ns"] for row in offers)
        need(dispatch >= offered and done >= max(row["end_ns"] for row in offers),
             "native-dispatch-after-logical-offer")
        native_span = max(row["end_ns"] for row in offers) - dispatch
        self.slots.append({
            "number": number, "offered_ns": offered, "dispatch_ns": dispatch, "done_ns": done,
            "helper_ns": max(0, done - dispatch - native_span),
            "backpressure_ns": dispatch - offered, "offers": normalized,
            "accepted_selected": copy.deepcopy(progress["selected"]),
        })
        self.host.record("history-slot", _encode({
            "phase": self.phase["id"], "slot": self.slots[-1],
            "state": progress.get("state")}, self.b), category="evidence",
            max_bytes=self.profile["slot_record_bytes"])
        if number == 99:
            self.publishers_done = True
        return {"number": number, "offered_ns": offered, "done_ns": done}

    def finish(self):
        need(not self.finished and len(self.slots) == 100, "exact100-slots-before-finish")
        self.publishers_done = True
        # Never stop readers merely because publishers finished: all forty
        # native responses, including their 40 first +40k warm queries, must end.
        errors = []
        for handle in self.handles:
            try:
                remaining = max(0, (self.origin["origin_perf_ns"]
                                   + self.capacity["reader_deadline_ns"] - time.perf_counter_ns()) / 1e9)
                self.a.wait(handle, remaining)
            except BaseException as error:
                errors.append(repr(error))
        need(not errors and not self.failures and len(self.readers) == 40, "all-finite-native-readers-complete")
        self.end_ns = time.perf_counter_ns()
        rows = [_observation(row, self.b, self.phase["root"])
                for row in self.ports.phase_call_observations(self.phase["id"])]
        need(len({r["token"] for r in rows}) == len(rows), "all-call-token-uniqueness")
        offered = [r for r in rows if r["input"].get("action") == "publish"]
        need(len(offered) == 100*self.phase["publishers"], "no-extra-or-hidden-publish-offers")
        builds = [r for r in rows if r["input"].get("action") == "packing"
                  and r["input"].get("operation") == "build"]
        cleanups = [r for r in rows if r["input"].get("action") == "packing"
                    and r["input"].get("operation") == "cleanup"]
        need(len(builds) == (100 if self.phase["online"] else 0)
             and len(cleanups) == (99 if self.phase["online"] else 0), "actual-matched-maintenance-counts")
        for row in builds + cleanups:
            need(row["exit"] == 0 and row["value"].get("status") == "ok", "maintenance-refusal")
        old = next(r for r in self.readers if r["reader"] == 0 and r["batch"] == 0)
        capture = self._perf(self.markers[0]["value"]["capture_qpc"])
        released = self._perf(old["header"]["pins_dropped"])
        publish_end = max(slot["done_ns"] for slot in self.slots)
        need(released - capture > 15000000000 and released < publish_end, "actual-old-pin-window")
        need(min(max(r["end_ns"] for r in self.readers if r["reader"] == n) for n in range(4))
             >= publish_end, "finite-reader-load-ended-before-publishers")
        native_progress = any(slot["done_ns"] > capture and slot["done_ns"] < released for slot in self.slots)
        read_progress = any(any(capture < self._perf(m[2]) < released for m in r["header"]["milestones"])
                            for r in self.readers if r["reader"] != 0)
        if self.phase["online"]:
            self.physical_states.append((time.perf_counter_ns(), self.h.state(self.phase["root"])))
        transitions = []
        prior = self.physical_states[0][1]
        for observed, state in self.physical_states[1:]:
            old, new = prior["store"]["physical"], state["store"]["physical"]
            if new["id"] != old["id"]:
                need(new["ordinal"] == old["ordinal"]+1 and state["catalog"] != prior["catalog"], "actual-mapping-transition")
                transitions.append((observed, new["id"]))
            prior = state
        need(len(transitions) == len(builds) and len({n for _, n in transitions}) == len(builds), "every-selected-mapping")
        mapping_ids = [n for t, n in transitions if t <= released]
        need(native_progress and read_progress and (len(mapping_ids) >= 2 if self.phase["online"] else not mapping_ids),
             "old-reader-real-publisher-reader-successors")
        metrics = []
        holds = []
        names = self.profile["metric_names"]
        closed(names, (*COUNTERS, "forbidden_global_operations", "unchanged_historical_body_opens"), "proved-counter-map")
        forbidden, unchanged = 0, 0
        for row in builds:
            raw_metrics = row["value"]["metrics"]
            need(all(type(n) is str and n in raw_metrics and integer(raw_metrics[n]) for n in names.values()), "source-mapped-counters")
            metrics.append({k: raw_metrics[names[k]] for k in COUNTERS})
            forbidden += raw_metrics[names["forbidden_global_operations"]]
            unchanged += raw_metrics[names["unchanged_historical_body_opens"]]
            holds.extend(raw_metrics["physical_activation_holds_ns"])
        need(forbidden == 0 and unchanged == 0, "actual-forbidden-or-unchanged-work")
        # The only case index is sealed AFTER boundary calls and case closure.
        # These draft SHA fields stay absent until its real completed hash exists.
        guard = self.ports.guard_prefix(self.phase["id"], self.start_ns, self.end_ns)
        need(guard.get("status") == "PROVISIONAL_UNQUALIFIED"
             and guard.get("guard_sha256") == self.b["guard_source"]["sha256"]
             and guard.get("final_receipt") is None, "actual031-prefix-not-final-PASS")
        self._finish_value = {
            "phase": self.phase["id"], "source_pin": self.h.final_pin,
            "binary_sha256": self.h.driver.binary_sha256, "receipt_sha256": None,
            "H_start": self.phase["H_start"], "H_end": self.phase["H_end"],
            "slots": copy.deepcopy(self.slots),
            "readers": {
                "initial_native_pids": [self.started[(i, 0)]["pid"] for i in range(4)],
                "calls": 40, "warm_queries": 40000, "first_queries": 40,
                "exhausted": False, "ended_before_publishers": False,
                "all_responses_exact": True, "raw_lossless": True, "rows_sha256": None,
                "old_reader": {
                    "pid": self.started[(0, 0)]["pid"], "captured_ns": capture, "released_ns": released,
                    "publishers_progress_during_hold": native_progress, "new_readers_progress_during_hold": read_progress,
                    "successor_mapping_ids": mapping_ids,
                    "held_generation_peak": max(len(s["work"].get("older", []))+1 for t,s in self.physical_states if t<=released),
                    "released_before_publishers_end": True, "exact_resume": True},
                "positive_native_overlap": True,
                "reader_rss_max": max(r["peak_rss"] for r in self.readers), "censors": 0},
            "maintenance": {
                "calls": len(builds) + len(cleanups), "builds": len(builds),
                "positive_publisher_overlap": any(r["start_ns"] < o["end_ns"] and o["start_ns"] < r["end_ns"]
                    for r in builds for s in self.slots for o in s["offers"]) if builds else False,
                "actual_mapping_changes": len(transitions),
                "zero_forbidden_under_global_exclusion": forbidden == 0,
                "unchanged_historical_body_opens": unchanged, "metrics": metrics,
                "activation_holds_ns": holds, "online_cleanup_calls": len(cleanups),
                "boundary_cleanup_calls": None if self.phase["online"] else 0,
                "final_unprotected_backlog": None,
                "raw_sha256": None},
            "terminal_receipts": [{k: v for k, v in t.items() if k != "raw_call_ref"} for t in self.terminals],
            "oracle": None, "closure": None, "guard_observation": guard,
            "frontend_censors": 0, "maintenance_refusals": 0, "hidden_retries": 0,
        }
        self.finished = True
        self.r.completed_phases[self.phase["id"]] = self
        return copy.deepcopy(self._finish_value)

    def close(self, primary=None):
        if self.closed:
            return
        errors = []
        if primary is not None or not self.finished:
            try:
                self.ports.abort_phase(self.phase["id"], repr(primary) if primary else "unfinished-phase")
            except BaseException as error:
                errors.append({"operation": "abort", "error": repr(error)})
        for number, handle in enumerate(self.handles):
            try:
                self.a.wait(handle, 5)
            except BaseException as error:
                errors.append({"operation": "wait", "reader": number, "error": repr(error)})
        try:
            self.ports.leave_phase(self.phase["id"])
        except BaseException as error:
            errors.append({"operation": "leave", "error": repr(error)})
        self.closed = True
        self.r.session = None
        try:
            self.host.record("history-reader-owned-close", _encode({
                "phase": self.phase["id"], "primary": repr(primary) if primary else None,
                "secondary": errors, "observed_callbacks": len(self.handles),
                "scope": "owned-children-IO-handles", "outer_Job_empty_claimed": False}, self.b),
                category="evidence", max_bytes=self.profile["closure_record_bytes"] if self.profile else 65536)
        except BaseException as error:
            errors.append({"operation": "close-record", "error": repr(error)})
        if primary is None:
            need(not errors, "owned-phase-close:" + repr(errors))


def _read_record(ports, ref, maximum=MAX_PHASE):
    ref = reference(ref, maximum)
    raw = ports.read_reference(copy.deepcopy(ref))
    need(type(raw) is bytes and len(raw) == ref["bytes"] and sha(raw) == ref["sha256"],
         "same-verified-record-buffer")
    return control_json(raw, maximum)


def _case_attachment(ports, ref, context, b, phase, kind):
    proof = _read_record(ports, ref)
    need(all(proof.get(k) == v for k, v in {
        "case_id": context["case_id"], "outer_run_id": context["outer_run_id"],
        "source_pin": b["history"]["source_pin"], "binary_sha256": b["history"]["binary_sha256"],
        "budget_sha256": b["history"]["budget_sha256"], "clock_domain": context["clock_domain"]
    }.items()), "actual-scoped-" + kind)
    need(integer(proof.get("start_ns"), context["case_start_ns"], context["case_end_ns"])
         and integer(proof.get("end_ns"), proof["start_ns"], context["case_end_ns"]),
         "actual-attachment-window:" + kind)
    if kind != "owned-drain":
        need(proof.get("owned_drain_sha256") == context["owned_drain_ref"]["sha256"],
             "actual-attachment-drain:" + kind)
    if kind == "owned-drain":
        need(proof.get("children_empty") is True and proof.get("callbacks_joined") is True
             and proof.get("scope") == "owned-children-IO-handles", "actual-owned-drain")
    elif kind == "checkpoint":
        need(proof.get("go") is True and proof.get("no_other_semantic_writer") is True,
             "actual-scoped-checkpoint")
    else:
        need(proof.get("processes_closed") is True and proof.get("handles_closed") is True
             and proof.get("python_callbacks_joined") is True, "actual-case-close")
    return proof


def _case_call_index(owner, phase, context, lease=None):
    ports, b = owner.ports, owner.b
    rows = ports.case_call_pairs(phase["id"])
    need(type(rows) is list and rows, "complete-closed-case-observations")
    profile = lease.profile if lease is not None else _profiles(b).initial_case_profile(
        owner.plan, phase["id"], b, owner.current)
    readers = lease.readers if lease is not None else []
    need(len(rows) <= profile["case_index_max_calls"], "source-derived-complete-case-call-bound")
    calls, tokens = [], set()
    previous = -1
    for row in sorted(rows, key=lambda x: x["raw_ref"]["begin"]):
        closed(row, ("raw_ref", "meta_ref"), "complete-case-call-pair")
        raw, meta_ref = reference(row["raw_ref"]), reference(row["meta_ref"])
        need(raw.get("schema") == CAPTURE and raw["run"] == context["outer_run_id"]
             and raw["call"] not in tokens
             and raw["begin"] > previous,
             "one-ordered-case-native-capture")
        # Native capture decoding belongs to the reviewed platform/finalizer.
        # Read the metadata through its actual ref, without copying native payloads.
        meta = _read_record(ports, meta_ref)
        need(meta.get("schema") == "native-call-meta-v2" and meta.get("raw_ref") == raw
             and meta.get("case_id") == context["case_id"] and meta.get("scope_id") == context["scope_id"]
             and meta.get("case_scope") == context["case_scope"], "actual-case-native-meta")
        start, end = meta.get("process_started_ns"), meta.get("process_done_ns")
        need(integer(start, context["case_start_ns"], context["case_end_ns"])
             and integer(end, start, context["case_end_ns"])
             and integer(meta.get("spawn_total_ns"), end-start), "actual-case-native-clock-window")
        previous = raw["begin"]
        need(integer(meta.get("qpc_frequency"), 1), "actual-native-clock-frequency")
        for key in ("clock_bridge_ref", "closure_ref"):
            reference(meta[key])
        tokens.add(raw["call"])
        calls.append({"raw_ref": raw, "meta_ref": meta_ref})
    need(all(r["raw_ref"]["call"] in tokens for r in readers),
         "all-forty-reader-calls-in-closed-case")
    body = {
        "schema": "history-native-case-index-v1", "phase_id": phase["id"],
        "outer_run_id": context["outer_run_id"], "source_pin": b["history"]["source_pin"],
        "binary_sha256": b["history"]["binary_sha256"], "budget_sha256": b["history"]["budget_sha256"],
        "case_start_ns": context["case_start_ns"], "case_end_ns": context["case_end_ns"],
        "case_id": context["case_id"], "scope_id": context["scope_id"],
        "case_scope": context["case_scope"], "call_count": len(calls),
        "calls": calls, "readers": copy.deepcopy(readers),
        "ready_refs": [reference(lease.markers[i]["ref"]) for i in range(4)] if lease else [],
        "origin_ref": reference(lease.anchor_ref) if lease else None,
        "owned_closure_ref": reference(context["owned_drain_ref"]),
        "checkpoint_ref": reference(context["checkpoint_ref"]),
        "case_closure_ref": reference(context["case_closure_ref"]),
    }
    raw = _encode(body, b)
    need(len(raw) <= profile["call_index_record_bytes"], "035-complete-case-index-width")
    ref = reference(owner.host.record("history-native-case-index-" + phase["id"], raw,
        category="evidence", max_bytes=profile["call_index_record_bytes"]), MAX_PHASE)
    need(ref["bytes"] == len(raw) and ref["sha256"] == sha(raw), "actual-completed-case-index")
    need(_read_record(ports, ref) == body, "case-index-same-buffer-reread")
    owner.case_indices[phase["id"]] = ref
    return ref


def seal_final_provisional(host, phase, evidence, runtime_bindings):
    """Seal once AFTER case close, including every boundary engine/checker call."""
    b = _bindings(runtime_bindings, execution=True)
    need(type(phase) is dict and type(evidence) is dict and evidence.get("phase") == phase["id"],
         "exact-phase-to-seal")
    owner = host.history
    need(isinstance(owner, HistoryRunner) and owner.b == b and owner.plan is not None
         and any(p == phase for p in owner.plan["phases"])
         and phase["id"] not in owner.closed_phases, "one-planned-final-phase-seal")
    owner._check_ports()
    ports = owner.ports
    lease = owner.completed_phases.get(phase["id"])
    need(lease is not None and lease.closed and lease.finished, "actual-finished-closed-lease")
    context = ports.seal_context(phase["id"])
    closed(context, ("case_closed", "case_id", "scope_id", "case_start_ns", "case_end_ns", "case_scope",
        "case_closure_ref", "checkpoint_ref", "owned_drain_ref",
        "outer_run_id", "host_pid", "host_creation_id", "clock_domain"), "after-case-seal-context")
    need(context["case_closed"] is True and context["case_id"] == b["case_catalog"]["history_case"]
         and context["scope_id"] == phase["id"] and digest(context["case_scope"], 32)
         and integer(context["case_start_ns"], 1, lease.start_ns)
         and integer(context["case_end_ns"], lease.end_ns)
         and context["case_end_ns"] <= time.perf_counter_ns(), "actual-complete-case-window")
    # Actual adapter nonce identifies this helper scope within the parent case.
    for key, kind in (("owned_drain_ref", "owned-drain"), ("checkpoint_ref", "checkpoint"),
                      ("case_closure_ref", "case-close")):
        _case_attachment(ports, context[key], context, b, phase, kind)
    need(evidence.get("closure") == {
        "processes_closed": True, "handles_closed": True, "callbacks_joined": True,
        "empty_observations": 2, "post_close_census": True, "scope": "owned-children-IO-handles"},
        "seal-after-actual-owned-closure")
    oracle = evidence.get("oracle")
    need(type(oracle) is dict and oracle.get("passed") is True
         and oracle.get("engine_audit_used") is False and oracle.get("actual_H") == phase["H_end"]
         and oracle.get("quiescent") is True and digest(oracle.get("receipt_sha256")),
         "final-full-oracle-attached")
    if phase["online"]:
        need(evidence["maintenance"]["boundary_cleanup_calls"] == 2
             and evidence["maintenance"]["final_unprotected_backlog"] == 0, "two-actual-boundary-passes")
    need(evidence["slots"] == lease._finish_value["slots"], "actual-lease-slot-links")
    # This is the case window, extending beyond the earlier reader lease window.
    guard = ports.guard_prefix(phase["id"], context["case_start_ns"], context["case_end_ns"])
    need(type(guard) is dict and guard.get("status") == "PROVISIONAL_UNQUALIFIED"
         and guard.get("final_receipt") is None
         and guard.get("phase_start_ns") == context["case_start_ns"]
         and guard.get("phase_end_ns") == context["case_end_ns"]
         and guard.get("guard_sha256") == b["guard_source"]["sha256"]
         and guard.get("budget_sha256") == b["history"]["budget_sha256"]
         and guard.get("finalizer_sha256") == b["finalizer_source"]["sha256"]
         and all(guard.get(k) == context[k] for k in ("outer_run_id", "host_pid", "host_creation_id", "clock_domain"))
         and guard.get("owned_drain_sha256") == context["owned_drain_ref"]["sha256"],
         "actual-full-case-031-prefix")
    owner.closed_phases[phase["id"]] = {"state": "sealing"}
    index_ref = _case_call_index(owner, phase, context, lease)
    inner = copy.deepcopy(evidence)
    inner["guard_observation"] = guard
    inner["readers"]["rows_sha256"] = index_ref["sha256"]
    inner["maintenance"]["raw_sha256"] = index_ref["sha256"]
    inner["receipt_sha256"] = index_ref["sha256"]  # REAL completed index; RAM pre-validation only.
    summary = _history(b).validate_phase_evidence(phase, inner)
    inner.pop("receipt_sha256")
    summary.pop("receipt_sha256")
    body = {
        "schema": BODY_SCHEMA, "status": "PROVISIONAL_UNQUALIFIED", "phase_id": phase["id"],
        "plan_sha256": owner.plan["plan_sha256"], "source_pin": b["history"]["source_pin"],
        "binary_sha256": b["history"]["binary_sha256"], "budget_sha256": b["history"]["budget_sha256"],
        "outer_run_id": context["outer_run_id"], "source_profile_sha256": b["profile_source"]["sha256"],
        "runtime_sha256": b["runtime_source"]["sha256"], "guard_sha256": b["guard_source"]["sha256"],
        "finalizer_sha256": b["finalizer_source"]["sha256"], "evidence": inner, "summary": summary,
        "owned_closure_ref": reference(context["owned_drain_ref"]),
        "checkpoint_ref": reference(context["checkpoint_ref"]),
        "case_closure_ref": reference(context["case_closure_ref"]), "native_call_index_ref": index_ref,
    }
    closed(body, BODY_KEYS, "selected-eighteen-field-body")
    raw = _encode(body, b)
    need(len(raw) <= MAX_PHASE, "final-phase-body-bound")
    ref = reference(host.record("history-final-provisional-" + phase["id"], raw,
                               category="evidence", max_bytes=MAX_PHASE), MAX_PHASE)
    need(ref["bytes"] == len(raw) and ref["sha256"] == sha(raw), "actual-written-final-body")
    need(_read_record(ports, ref) == body, "final-body-same-buffer-reread")
    index = {"id": phase["id"], "body_ref": ref}
    external = reference(ports.append_phase_index(copy.deepcopy(index)))
    need(_read_record(ports, external) == index, "actual-external-index-identity")
    owner.closed_phases[phase["id"]] = {"body": ref, "index": external}
    return {"body": ref, "index": external, "phase": phase["id"], "status": "PROVISIONAL_UNQUALIFIED"}


def sealed_phase_evidence(host, seal, runtime_bindings):
    b = _bindings(runtime_bindings)
    closed(seal, ("body", "index", "phase", "status"), "sealed-phase-envelope")
    need(seal["status"] == "PROVISIONAL_UNQUALIFIED", "no-in-host-final-PASS")
    owner = host.history
    need(isinstance(owner, HistoryRunner) and owner.b == b and
         owner.closed_phases.get(seal["phase"]) == {"body": seal["body"], "index": seal["index"]},
         "actual-owner-sealed-envelope")
    ports = owner.ports
    ref = reference(seal["body"], MAX_PHASE)
    need(_read_record(ports, seal["index"]) == {"id": seal["phase"], "body_ref": ref},
         "external-index-identifies-exact-body")
    body = _read_record(ports, ref)
    closed(body, BODY_KEYS, "stored-selected-eighteen-field-body")
    need(body["schema"] == BODY_SCHEMA and body["status"] == "PROVISIONAL_UNQUALIFIED"
         and body["phase_id"] == seal["phase"] and body["plan_sha256"] == owner.plan["plan_sha256"]
         and body["source_pin"] == b["history"]["source_pin"]
         and body["binary_sha256"] == b["history"]["binary_sha256"]
         and body["budget_sha256"] == b["history"]["budget_sha256"]
         and body["source_profile_sha256"] == b["profile_source"]["sha256"]
         and body["runtime_sha256"] == b["runtime_source"]["sha256"]
         and body["guard_sha256"] == b["guard_source"]["sha256"]
         and body["finalizer_sha256"] == b["finalizer_source"]["sha256"]
         and "receipt_sha256" not in body["evidence"] and "receipt_sha256" not in body["summary"],
         "actual-sealed-body-not-prefix")
    value = copy.deepcopy(body["evidence"])
    value["receipt_sha256"] = ref["sha256"]  # Actual external bodySHA; RAM projection only.
    phase = next(p for p in owner.plan["phases"] if p["id"] == seal["phase"])
    summary = _history(b).validate_phase_evidence(phase, value)
    summary.pop("receipt_sha256")
    need(summary == body["summary"], "actual-recomputed-summary")
    return value
