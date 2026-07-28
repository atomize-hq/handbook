# P2A native-platform feasibility

Status: non-production probe complete; durable evidence is independently
review-gated. No production adapter or P2P work began.

## Authority and boundary

- Source handoff:
  `20260727T215133Z--HCM-2-4--orchestration--p2-dual-ceiling-planning-completed`.
- P2S reviewed commit:
  `755acc56fad6cc99c016dc58f94d310c84f98c58`.
- P2S reviewed subject:
  `sha256:7b1d0d4444b84613334625191f2d7272e2a866a7ba38d1b67aca875bc78c5f04`.
- P2A scope: the smallest local, non-production feasibility probe only.
- No dependency, Cargo file, lockfile, unsafe code/policy, production module,
  compiler/runtime source, public API, or persistent probe test was added or
  changed.

The one temporary integration-test probe was deleted immediately after its
1/1 pass. The durable P2A delta is this evidence plus directly coupled status
text and immutable review/closeout artifacts.

## Local platform observation

The host reported:

```text
OS: Microsoft Windows NT 10.0.26200.0
host: x86_64-pc-windows-msvc
webauthn.dll: C:\WINDOWS\System32\webauthn.dll
file/product version: 10.0.26100.7171
Windows SDK header:
C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um\webauthn.h
```

The local SDK declares `WebAuthNAuthenticatorGetAssertion`,
`WebAuthNCancelCurrentOperation`, and `WEBAUTHN_ASSERTION` fields for exact
authenticator data, signature bytes, and the selected credential. This is
positive evidence that the operating system has a native WebAuthn surface
capable in principle of returning the material required by the existing ES256
verifier.

It is not a safe callable primitive in the current workspace:

- no `webauthn`, FIDO, CTAP, or HID binding is a direct or transitive
  application dependency;
- the two transitive `windows-sys` versions are present only through
  `file-id`, `tempfile`, and terminal styling and do not expose an approved
  WebAuthn adapter here;
- the production compiler still injects only
  `UnavailableNativeAuthenticatorPortV1`; and
- invoking the native API or direct CTAP HID transport would require a new
  dependency/Cargo selector or new FFI/unsafe policy, both forbidden by P2A.

The read-only PnP inventory found no present entity whose name or device ID
matched `FIDO`, `Security Key`, `Authenticator`, or `WebAuthn`. That does not
disprove a Windows Hello platform authenticator, but it means no external
authenticator was safely identified for a live ceremony.

## Observed result matrix

| Required observation | Exact local result | Evidence strength |
| --- | --- | --- |
| Safe native authenticator primitive | Windows WebAuthn DLL/header are present, but no approved safe Rust binding or workspace adapter exists. The primitive is therefore unavailable to the current selector. | Platform availability only; no call made |
| CTAP2.1 request/response transport | Engine request encoding and response decoding replay, but no live transport exists. The engine port exchanges raw CTAP2.1 CBOR while Windows WebAuthn is a semantic API, so a reviewed translation boundary is still required. | Frozen transcript, not live transport |
| Deterministic credential selection | The engine sorts eligible credential IDs, rejects duplicates, and refuses more than 64. Native selection among an allow-list was not observed. | Deterministic pre-native selection only |
| User presence and verification | Requests set both `up` and `uv` true; the verified transcript returns flags `0x05`, and missing/invalid behavior refuses. No local user ceremony ran. | Codec/verifier proof only |
| Cancellation | CTAP status `0x2d` maps exactly to non-retryable `authenticator_user_cancelled`; the SDK exposes a cancellation API. Native cancellation timing/HRESULT behavior was not invoked. | Status-map/API-presence proof only |
| Malformed response and native status/error mapping | All frozen malformed/status responses refuse and every nonzero CTAP byte maps exactly once. Windows HRESULT/native-error-to-port mapping is unimplemented and unobserved. | Complete CTAP map; no native map |
| ES256 assertion material | The frozen success transcript verifies ES256 and returns 37-byte authenticator data, credential ID, flags `0x05`, sign count `7`, and no attested data/extensions. The SDK assertion shape exposes authenticator data/signature/credential fields, but no live assertion was returned. | Verifier plus API-shape feasibility |
| Unavailable-port zero-write behavior | A temporary compiler integration probe initialized repository identity, snapshotted every path/byte, invoked the real public adapter with its default unavailable port, and proved an `AUTHENTICATOR_UNAVAILABLE` refusal, empty `changed_paths`, and a byte-identical before/after repository snapshot. | Direct 1/1 non-production adapter proof |

## Commands and results

```text
cargo test -p handbook-engine --test hcm_2_2_authenticator_security -- --nocapture
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

cargo test -p handbook-compiler --test hcm_2_4_p2a_temporary_native_probe -- --nocapture
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The 9/9 wall proves deterministic codec/selection behavior, strict
make-credential decoding, malformed assertion refusal, all-byte CTAP status
mapping, ES256 assertion verification, and closed unavailable-port refusal.
It remains supporting evidence only and does not prove a production native
adapter. The temporary 1/1 test proved filesystem zero-write behavior around
the existing public compiler adapter and was then discarded; `git status`
contains no probe path.

Read-only platform/dependency probes additionally checked:

```text
Test-Path C:\WINDOWS\System32\webauthn.dll
Get-Item ...\webauthn.dll | inspect file/product version
Get-CimInstance Win32_PnPEntity | filter FIDO/security-key/authenticator names
cargo tree -p handbook-engine
cargo tree -p handbook-compiler
cargo tree -i windows-sys@0.60.2 --workspace
cargo tree -i windows-sys@0.61.2 --workspace
inspect local Windows SDK webauthn.h declarations
```

## Feasibility conclusion

A Windows production adapter appears architecturally plausible because the
native API and required assertion-material shape are present. It is not yet
demonstrated or implementable within current authority. The missing safe
binding/transport, live device ceremony, native cancellation/HRESULT mapping,
credential-selection behavior, and UP/UV observation are material unresolved
items—not evidence to infer away.

P2A therefore returns **conditionally feasible, presently blocked**. It did
not prove a production native adapter and does not authorize one.

## Exact second human decision

Before P2P, the human must make one explicit, fingerprinted decision that
either:

1. authorizes a separately bounded production-native-adapter packet and names
   the exact platform approach (`Windows WebAuthn` semantic translation or
   direct CTAP2.1 transport), safe binding/dependency and version/features,
   permitted Cargo/lockfile changes, UI/window owner, cancellation contract,
   HRESULT/native-status mapping, credential-selection rule, supported-device
   matrix, and unsafe-code policy; or
2. names an already completed, independently reviewed producer/native-adapter
   dependency that satisfies those same contracts.

The first choice authorizes only that adapter packet. P2P still requires a new
explicit selector after the adapter is review-clean. Without either choice,
P2P remains blocked.

## True stop

No production adapter, P2P, P2R, P2I, P2V, P4, P6, Phase 2 exit, or another
slice began. The unavailable/partial result is the required P2A outcome, not a
reason to widen the packet.
