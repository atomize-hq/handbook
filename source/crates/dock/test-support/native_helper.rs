use handbook_contracts::{EvidenceObservationV1, ExactBindingV1, ExactRefV1, FingerprintV1};
use handbook_dock::*;
use serde_json::json;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::time::Duration;

fn fingerprint(hex: char) -> FingerprintV1 {
    FingerprintV1::parse(&format!("sha256:{}", hex.to_string().repeat(64))).unwrap()
}

fn binding(reference: &str, hex: char) -> ExactBindingV1 {
    ExactBindingV1::new(ExactRefV1::parse(reference).unwrap(), fingerprint(hex))
}

fn read_request() -> DockProcessRequestV1 {
    let mut bytes = Vec::new();
    std::io::stdin().read_to_end(&mut bytes).unwrap();
    assert_eq!(bytes.last(), Some(&b'\n'));
    serde_json::from_slice(&bytes[..bytes.len() - 1]).unwrap()
}

fn completed(request: &DockProcessRequestV1, one_candidate: bool) -> DockProcessResultV1 {
    let candidates = if one_candidate {
        vec![DockCandidateObservationV1::new_with_observation(
            request.run().claim_ids()[0].clone(),
            request.run().cases()[0].case_id().clone(),
            request.requested_evidence_kinds()[0].clone(),
            binding("handbook.fact.schema-conformance@1.0.0", 'b'),
            json!({"valid": true}),
            EvidenceObservationV1::Satisfied,
        )
        .unwrap()]
    } else {
        vec![]
    };
    DockProcessResultV1::completed(
        request,
        DockClaimPartitionV1::new(request.run().claim_ids().to_vec(), vec![], vec![]).unwrap(),
        candidates,
        vec![],
        request.run().requested_resolution().clone(),
    )
    .unwrap()
}

fn completed_at_result_body_limit(request: &DockProcessRequestV1) -> DockProcessResultV1 {
    fn with_padding(request: &DockProcessRequestV1, padding: usize) -> DockProcessResultV1 {
        let candidate = DockCandidateObservationV1::new_with_observation(
            request.run().claim_ids()[0].clone(),
            request.run().cases()[0].case_id().clone(),
            request.requested_evidence_kinds()[0].clone(),
            binding("handbook.fact.schema-conformance@1.0.0", 'b'),
            json!({"padding": "x".repeat(padding)}),
            EvidenceObservationV1::Satisfied,
        )
        .unwrap();
        DockProcessResultV1::completed(
            request,
            DockClaimPartitionV1::new(request.run().claim_ids().to_vec(), vec![], vec![]).unwrap(),
            vec![candidate],
            vec![],
            request.run().requested_resolution().clone(),
        )
        .unwrap()
    }

    let baseline = with_padding(request, 0);
    let baseline_body = baseline.framed_json_bytes().unwrap().len() - 1;
    let target = request.effective_limits().result_bytes();
    let padding = target.checked_sub(baseline_body).unwrap();
    let exact = with_padding(request, padding);
    assert_eq!(exact.framed_json_bytes().unwrap().len(), target + 1);
    exact
}

fn write_result(result: &DockProcessResultV1) {
    std::io::stdout()
        .write_all(&result.framed_json_bytes().unwrap())
        .unwrap();
}

#[allow(clippy::zombie_processes)]
fn spawn_output_holder() {
    Command::new(std::env::current_exe().unwrap())
        .arg("hold-only")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
}

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let mode = arguments.first().map(String::as_str).unwrap_or_default();
    if mode == "ignore-stdin" {
        std::thread::sleep(Duration::from_millis(2_000));
        return;
    }
    if mode == "hold-only" {
        std::thread::sleep(Duration::from_millis(2_000));
        return;
    }
    let request = read_request();
    match mode {
        "completed-zero" => write_result(&completed(&request, false)),
        "completed-one" => write_result(&completed(&request, true)),
        "completed-exact-limit" => write_result(&completed_at_result_body_limit(&request)),
        "refused" => write_result(
            &DockProcessResultV1::refused(
                &request,
                DockProcessRefusalReasonV1::ObservationUnavailable,
                vec![],
            )
            .unwrap(),
        ),
        "verify-host" => {
            let environment: Vec<_> = std::env::vars().collect();
            let cwd = std::env::current_dir().unwrap();
            let private_cwd = cwd.read_dir().unwrap().next().is_none();
            let exact_environment = environment
                == vec![(
                    "HANDBOOK_DOCK_PROTOCOL".to_owned(),
                    DOCK_PROCESS_PROTOCOL_REF_V1.to_owned(),
                )];
            let exact_arguments = arguments.get(1).map(String::as_str) == Some("literal;not-shell");
            if private_cwd && exact_environment && exact_arguments {
                write_result(&completed(&request, false));
            } else {
                write_result(
                    &DockProcessResultV1::refused(
                        &request,
                        DockProcessRefusalReasonV1::InternalRefusal,
                        vec![],
                    )
                    .unwrap(),
                );
            }
        }
        "invalid-utf8" => std::io::stdout().write_all(&[0xff, b'\n']).unwrap(),
        "invalid-json" => std::io::stdout().write_all(b"not-json\n").unwrap(),
        "duplicate-json" => std::io::stdout()
            .write_all(b"{\"status\":1,\"status\":2}\n")
            .unwrap(),
        "noncanonical" => {
            let mut bytes = serde_json::to_vec_pretty(&completed(&request, false)).unwrap();
            bytes.push(b'\n');
            std::io::stdout().write_all(&bytes).unwrap();
        }
        "extra-output" => {
            write_result(&completed(&request, false));
            std::io::stdout().write_all(b"extra").unwrap();
        }
        "oversized-output" => {
            std::io::stdout()
                .write_all(&vec![b'x'; 128 * 1024])
                .unwrap();
        }
        "stderr-secret" => {
            std::io::stderr()
                .write_all(b"credential=DO_NOT_DISCLOSE proxy=http://secret")
                .unwrap();
            write_result(&completed(&request, false));
        }
        "stderr-overrun" => {
            std::io::stderr()
                .write_all(&vec![b'x'; 128 * 1024])
                .unwrap();
            write_result(&completed(&request, false));
        }
        "wrong-binding" => {
            let result = completed(&request, false);
            let mut value = serde_json::to_value(result).unwrap();
            value["request"]["fingerprint"] = json!(fingerprint('f'));
            let mut bytes = serde_json_canonicalizer::to_vec(&value).unwrap();
            bytes.push(b'\n');
            std::io::stdout().write_all(&bytes).unwrap();
        }
        "sleep" => {
            let millis = arguments.get(1).unwrap().parse().unwrap();
            std::thread::sleep(Duration::from_millis(millis));
            write_result(&completed(&request, false));
        }
        "mark-and-sleep" => {
            std::fs::write(arguments.get(1).unwrap(), b"launched").unwrap();
            let millis = arguments.get(2).unwrap().parse().unwrap();
            std::thread::sleep(Duration::from_millis(millis));
            write_result(&completed(&request, false));
        }
        "count-and-complete" => {
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(arguments.get(1).unwrap())
                .unwrap()
                .write_all(b"x")
                .unwrap();
            let millis = arguments.get(2).unwrap().parse().unwrap();
            std::thread::sleep(Duration::from_millis(millis));
            write_result(&completed(&request, true));
        }
        "hold-stdout-open" => {
            spawn_output_holder();
            write_result(&completed(&request, true));
        }
        "nonzero" => std::process::exit(7),
        "crash" => panic!("test-only native helper crash"),
        _ => std::process::exit(64),
    }
}
