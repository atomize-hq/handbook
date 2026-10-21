# Quota V2 freeze2: Q1/Q2 source review

**SOURCE-ONLY disposition: INTEGRATE the exact freeze2 five-file set. Runtime HOLD; full V6/native/real-store qualification INCOMPLETE.** Both Q1 and Q2 are addressed; no additional blocking defect was established. This approves source handoff for subsequent bounded-guard/admission review, not execution or qualification. FINAL_SOURCE_PIN remains None and blocks full V6 before store reads.

Freeze receipt: d5a7b59ea0a4d05ba59e901a93c0a5e24448878047b38cb323a15d107024db79. New sources total44,806bytes within45,056: quota18,418; controls10,234; semantic10,071; V6 4,036; plan2,047. The companion evidence contains all13full path/size/SHA entries. All five new sources were fully read and compared to the exact old five-file capsule in RAM; all13new/transitive bindings matched at first/end observations. Semantic V2 and all eight transitive sources remain byte-identical; V6 only updates the quota companion hash; the plan updates static case counts.

New capsule SHA93533c3b45ce7ffcdd8f7e333e9ba3f42f2b65172ba8fd79ff17f134bda689b3 and its decoded9dd348645fe8f98247febb626f1eac3eba24aa441d8ebfa7a1b74ee9f1a87a39 match. All five embedded decoded source sizes/hashes match. Old80cea receipt/89685 capsule and previous REMEDIATE reports are preserved. The prior report's temporary17,828-byte formatting overrun remains an incident; its final16,309-byte size does not erase that history. This pair was byte-counted in RAM before any write.

No module import, AST compile, control/test/oracle/native/engine/Git execution, fixture/store access or source edit occurred. Only source/receipt reads, archive/data comparisons, hashing, census and these two reports were authorized. Last source checks preceded the static-complete/freeze-release notice; no source was read afterward. Proposed structured standard/owner contracts and fixed clocks remain unchanged. Manual impact remains HIGH/graph UNKNOWN, with no graph/indexing claim.

## Q1 addressed

quota_v2_oracle.py:175 explicitly maps the filename terminal-control-stage to emitted phase terminal-control, while the receipt slot maps to terminal-receipt-stage. Lines184/194 use that emitted prefix for both checkpoint validation and phase extraction. This matches the previously inspected frozen Rust write_stage emitter at lifetime.rs:249-257 and terminal_control caller:265-266. Owner/source/qualified-schedule/hash, prefix, exact half-write and complete-byte checks remain intact.

Controls:83 now prepares the actual replacement registry's first half, rather than an arbitrary30-byte prefix; its stage helper supplies terminal-control-partial. The bytes are nonempty and exactly floor(expected length/2), with registry field order and typed selector normalization matching the oracle. Controls:120 deliberately supplies the invented terminal-control-stage-created name and must refuse at checkpoint validation. Existing receipt-stage/reused-created prefix handling is retained. The other native phase names are recognized by the explicit prefix grammar; they are not separately claimed as executed controls.

## Q2 addressed

quota_v2_oracle.py:161 now enforces used+128KiB == charge <= original_allowance <= LIMIT-MARGIN. This supplies the necessary historical new-V2 admission bound independently of optional preowner evidence. It does not reconstruct historical I/O, change current aggregate credit, or admit legacy owners to the terminal path.

Controls:119 changes only the committed receipt's original allowance to33GiB while keeping the small valid charge; this directly targets the former false-pass. Controls:118 supplies a feasible pending owner with both active allowance and receipt original allowance equal LIMIT-MARGIN: current terminal counters remain0 while the published receipt is pending, so active allowance+control margin equals exactly32GiB and no terminal double count is introduced. The candidate/used/charge remain valid. The upper-bound comparison also statically excludes LIMIT-MARGIN+1, though a separate+1 control is not present. Optional preowner comparison remains unchanged.

## Scope and controls

The diff retains the earlier10positive/57negative case coverage, shortening synthetic identifiers, B payload/path and invalid prefix junk to fit unchanged source caps. It adds one positive and two negatives, with an exact source assertion of11positives/59negatives. These are70unexecuted definitions. The planned MemorySnapshot/Authority tests cover parser behavior, not real accepted graphs, NativeSnapshot handles, native aliases, stage scheduling or V6's complete semantic composition. No PASS is inferred from source or the receipt's AST/source-shape checks.

The quota proof wording still describes used as an attestation. Its redundant final_source_pin=None output field was removed; the full V6 wrapper still adds the selected final pin when one is legitimately bound. The eight-dependency byte/hash match preserves the previously reviewed independent v5/page/cohort, legacy/tree, semantic/navigation and intent paths. No format-owner migration or final paced engine binding is granted. The old guard/output forecast remains withdrawn.

## Admission and handoff

Independent prewrite census under exact META9 membership-v4 SHA024a23d8304de4dec44332b1c6d686031ffc1dbc9f5a30e678f2bae03d5dfe0e counted all P, closed Landing roots/subtrees with stream controls excluded, and all20promoted harness files once per pathname:8,580,281bytes/1,034files. With12,288review reserve and307,200future reserve, prospective8,899,769 <9,437,184cap. This is the whole declared membership, not P-only or the withdrawn8MiB forecast.

INTEGRATE is limited to this exact source set. Runtime stays HOLD until the new hard-bounded controller/output source has its separate review and an actual finite run has fresh census/GO. Native/full-store V2 and final-pin oracles remain pending. Preserve all prior failures, receipts, source caps and clock limits.
