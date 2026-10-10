# Historical graph consultation: captured answer

Date: 2026-10-10. Model shown by UI: GPT-6, Pro effort.
Task: retrieve the user's prior graph and file-based graph ideas from accessible ChatGPT account context.
Conversation: https://chatgpt.com/c/6aca7908-5afc-83e9-bb54-8fd9b24cb882
Exact prompt: [history-prompt.md](history-prompt.md).
Capture: completed assistant response, copied through its rendered response control. ChatGPT's interactive file citations were copied as source labels, not portable URLs; use the linked original conversation to inspect them. Reported search coverage and historical attributions below are the consultant's statements, not an independently enumerated account archive. A temporary connection interruption resolved before capture.

Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

---

# Recovering Your File-Based Graph Ideas

**Yes—there is substantial prior work to recover.** The strongest findings are not merely past mentions of graph databases. They include a file-canonical graph concept you described in May 2026, an actual **`memsrc` / Memory-as-Source** specification, a **C3 file-backed graph registry**, and later **Handbook and Unified Agent API evidence designs** with immutable records, derived indexes, content identities, and dependency-specific invalidation. The `memsrc` and support-depth documents provide particularly concrete evidence. memsrc_memory_as_source_poc_spec Pasted text(20260929-023608)

The important qualification is that these are **several related lines of work, not a single recovered, fully implemented version of today’s proposed Handbook graph**.

## 1. Access and search coverage

### What I could actually search

| Source | Access and investigation | Limitations |
|---|---|---|
| **Prior conversation context** | Used the account’s personal-context retrieval tool repeatedly, first broadly and then following discovered names, dates, and terminology. Results distinguished historical user messages from assistant messages. | This is retrieval over available context, **not an enumerable transcript archive**. Most results were attributed summaries; a few included short quoted excerpts. Most supplied neither original chat titles nor links. |
| **Saved-memory context** | Reviewed the memory context available to this conversation as leads. | No separate, complete saved-memory export or exhaustive memory-listing operation was exposed. The principal findings below come from conversation retrieval and files, not a claim that saved memories were fully enumerated. |
| **Library and accessible project files** | Searched Library content broadly, read strong matches, compared successive C3 drafts, and inspected the accessible `/substrate` folder through its returned pagination. | A Library folder is not proof of access to all native ChatGPT Projects, their instructions, or every associated conversation. |
| **Original documents** | Fully read the recovered `memsrc` specification, one developed C3 design, the March Substrate architecture draft, and the standalone HCM-4.6 snapshot document. Inspected relevant sections of support-depth drafts and the HCM audit packet. | Repository implementation claims in these documents are documentary evidence—not code or test results independently verified in this investigation. |
| **Project archives** | Located Substrate ZIP archives and attempted both reading and raw-byte access. | The reads returned no readable content; the raw-copy attempt was denied because the Project file lacked an authorized materialization path. Their internal repository files therefore remained uninspected. substrate copy Archive-7-11-2026 |
| **Native Projects and complete chat enumeration** | No direct native-Projects enumerator, complete chat-list tool, or general full-transcript opener was exposed. | I cannot report a percentage of your account covered or claim an exhaustive search. |

I did not substitute web research for your historical sources. A connected external Drive mount was visible, but it was not treated as a substitute for ChatGPT history or silently included as an additional research corpus.

The searches included your supplied storage, graph, identity, and invalidation terminology, then followed discovered terms including **Spec / SpecGraph, `.specs`, `.specignore`, Memory-as-Source, `memsrc`, PageIndex, LadybugDB, Cognitive Twin Substrate, C3, Forge, Lift, RepoSnapshot, unit passports, Snapshot Memory, and support-depth dependency bindings**. Ranked file searches often returned repeated passages from the same document; those were not counted as independent corroboration.

**Evidence distinctions used below:**

- **Original document:** actual file text inspected and cited.
- **Retrieved conversation excerpt:** a short passage returned as historical message text, not a full transcript I opened.
- **Retrieved conversation summary:** attributed historical paraphrase, with its returned date.
- **Present inference:** my interpretation of relevance or continuity, not something attributed to your past self.

Dates attached to conversation retrieval below use the returned UTC date. Where a chat title was not exposed, I use a descriptive heading rather than invent one. No sidebar-title editing control was exposed; **“Recovering Your File-Based Graph Ideas”** is the descriptive title for this consultation.

## 2. Evidence-backed inventory

### A. Spec: graph-linked specification files with separate Git history  
**May 30, 2025 — retrieved conversation excerpts and summaries**

This is the earliest strong file-backed graph concept recovered.

**What you proposed:** specification files with dependency and context links to other specification files; spec-specific history useful for selecting recent changes, context, and previous attempts; and Qdrant-related retrieval coordinates associated with that structure.

The retrieved user messages distinguish a particular repository design:

```text
.git         primary repository history
.spec        bare, spec-only Git data
.specignore  spec-specific exclusion rules
.specs       a documentation/specification mirror of the codebase
```

At **13:16:32 UTC**, you explicitly corrected the assistant:

> “I don’t want to put git inside of spec. I want to FORK git”

The layout above was returned from your **14:08 UTC** message; graph-linked specs were returned from **13:09 UTC**.

**What the assistant supplied:** **“SpecGraph”** was the assistant’s label for the concept. I did not recover evidence establishing it as a name you independently selected.

**What this establishes:** you were already considering a graph whose operational material consisted of ordinary files, with graph relations and a distinct version-history model.

**What it does not establish:** Git-versioned specification files are not, by themselves, a defined immutable-node/versioned-edge protocol. The retrieval did not expose that level of design.

**Source limitation:** no original chat title, usable chat link, or corresponding original spec file was recovered.

---

### B. Belief-centric memory, mountable subgraphs, and embedded graph databases  
**July 31 and November 27, 2025 — retrieved conversation summaries**

These are related interests, but they need to remain separate from the database-free file-compilation concepts.

**What you proposed in July:** a persistent memory/world model involving a **Core/Soul Node**, a **Belief Ring**, themed experience subgraphs, reinforcement or decay, and evolving relationships. Retrieved messages also describe recursive or mountable subgraphs and a combined vector/graph/relational memory direction.

**What the assistant suggested:** file-backed subgraphs, `pointer_to_subgraph` references, centroid or summary representations, and lazy mounting/unmounting. Kuzu was discussed as a possible implementation. Those storage mechanisms are recovered **assistant recommendations**, not all direct user specifications.

**What you asked in November:** the **November 27, 18:42 UTC** retrieval describes your investigation of Rust-native, filesystem-based graph databases comparable to Kuzu/SQLite, followed by questions about vector embeddings and LadybugDB. The assistant then compared embedded alternatives and recommended a possible Kuzu-continuation path.

This matters because it shows that your historical preference was **not uniformly “no database.”** Sometimes the requirement was an embedded, local, file-backed database rather than a server database. Elsewhere, you explicitly wanted graph relations without a separate storage mechanism.

**An important attribution correction:** one retrieved July thread records an assistant importing belief/soul concepts from another conversation into a proposal it called **“The Forge Model,”** and you objected to that cross-chat import. I therefore do **not** merge that proposal with the later **Forge/C3** project or present the brain-model discussion as one continuous architecture.

**Source limitation:** these results did not expose original titles or links. The database comparisons are historical recommendations, not a current assessment of those products.

---

### C. Substrate’s semantic-unit source graph  
**“Substrate — High-Level Technical Architecture,” v0.1, Draft, dated March 25, 2026 — original document**

This is a substantial original artifact:

`substrate_high_level_technical_architecture_v0.1.md`

It describes a semantic-unit system within Substrate, with Collider providing editing and navigation. I am preserving the document’s historical use of those names rather than assuming it is identical to every later Substrate architecture. substrate_high_level_technical\_…

**Documented design:** authors edit **`*.unit.yaml` and `*.test.yaml`**, validated through CUE. A unit carries stable identity, intent, contract, implementation body, local verification, and explicit dependencies. The graph resolver establishes unit dependencies, declared test links, reverse references, and relevant cycle diagnostics. substrate_high_level_technical\_… substrate_high_level_technical\_…

Its illustrative layout separates authored files from generated artifacts:

```text
schema/
units/
tests/
build/
  generated/
  passports/
  docs/
  diagnostics/
```

The source files own authored truth; embeddings, fingerprints, coverage counts, generated code, and drift scores belong in build output. substrate_high_level_technical\_…

Two especially useful passages are:

> “declared links — what source files say should be covered”

versus

> “observed links — what runtime evidence shows was executed”

and:

> “Passports are versioned derived data.” substrate_high_level_technical\_…

**Attribution:** this is an actual historical design artifact. Retrieved conversation context associates the architecture write-up with an assistant response and your follow-up requests for technical background and a roadmap. The file alone does not establish that every individual design choice originated with you or was approved.

**Important limit:** incremental change detection was listed as later support. This document is evidence for file-authored graph construction and evidence-rich derived views, not an implemented incremental invalidation algorithm. substrate_high_level_technical\_…

---

### D. Lift: repository snapshots, graph closure, and a boundary against repeated filesystem access  
**April 13–18, 2026 — retrieved conversation summaries**

This is particularly relevant to your question about redundant low-level filesystem checks, but its provenance is weaker than the original documents above.

**What you specified:** the **April 18** retrieval attributes to you a division in which **Lift owns repository/code intelligence**—dependency maps, existing-code leverage, test-framework detection, codepath inventory, and workspace-local prior-artifact resolution—while peer components own other workflow responsibilities.

**What the assistant proposed:** the **April 13** material describes a deterministic repository snapshot, language adapters, graph/scope traversal using breadth-first closure, provenance and content fingerprints, and stable traversal behavior. A later response that day proposed a snapshot-provider boundary **intended to prevent downstream filesystem access**.

That is a meaningful historical antecedent to:

> Acquire and establish a bounded view once, then let downstream graph operations consume that view instead of independently reopening the repository.

That sentence is **my synthesis of the retrieved assistant proposal**, not a recovered quotation from you.

Later retrieved material also distinguishes roles as **“effort defines the graph; exec runs the graph; lift informs the graph.”** That formulation was assistant-produced.

**Limits:** the relevant original Lift design/code files were not recovered as readable Library text. The available project archives could not be opened. Consequently, I cannot establish which snapshot guarantees or traversal mechanisms actually shipped.

---

### E. Your file-canonical graph/self-model concept  
**May 24–25, 2026 — retrieved conversation summaries and a short excerpt**

**This is the closest recovered match to your explicit preference for proving a graph directly over files before requiring another storage system.**

The **May 24, 14:45 UTC** retrieval attributes to you a proposal involving:

- Typed, linted, validated Markdown, YAML, TOML, and JSON documents in manually navigable directories.
- Graph structure/runtime derived from those files.
- Hot local work with cold S3/object storage.

In the **May 25, 17:52 UTC** message, you referred to PageIndex and LadybugDB and wanted upfront graph relations using JSON “or whatever,” through files:

> “without a seperate storage mechanism”

That phrase was returned as a historical user excerpt; the rest above is retrieval paraphrase.

**Assistant elaboration:** the following response proposed typed files and folders as canonical material, PageIndex-style navigation, JSON/JSONL artifacts first, LadybugDB only when needed as a derived runtime/cache, and manifests/hashes for reproducibility and cache invalidation.

**A different assistant branch must not be mistaken for your settled decision:** another May 24 response called the system **“Cognitive Twin Substrate”** and recommended a file-first vault with **Postgres as a hot memory core**, LadybugDB as a graph/index cache, and object storage for cold immutable blobs. That name and stack were assistant suggestions. I found no evidence that you adopted that complete stack.

**What is historically established:** you explicitly explored **file-native graph relationships without a separate store**.

**What remains unavailable:** an original full design document, the complete surrounding exchange, an original chat title/link, and proof of an adopted implementation. The detailed manifest, hashing, hydration, and caching design belongs to the assistant’s elaboration unless separately evidenced.

---

### F. Forge/C3: a file-backed, typed graph registry with generated relationships  
**May 24–25, 2026 — original successive design documents plus conversation summaries**

The actual recovered title is **“C3 Extraction Inventory Design.”** Several successive versions survive as `Pasted text(21).txt` through later numbered files. The design identifies `source_project: forge`; its example extraction timestamp is May 24. Retrieved conversations independently place the discussion on May 24–25. Pasted text(24)

The intended package separated:

```text
docs/c3_extraction_inventory_design.md
anvil/harness/inventory/c3_inventory.yaml
tests/fixtures/harness/c3_microstrategies/
```

The registry was meant to drive extraction coverage, rebuild queues, proof gating, compatibility, and promotion—not merely document nodes. Pasted text(24)

**What the documents specify:** edges, ports, conditions, loops, transforms, failures, artifacts, trace events, and proofs are first-class where they carry independent behavior. References must resolve to real inventory items or integrity-checked catalogs. A compact supporting excerpt:

> “Graph topology is not allowed to hide inside prose strings.” Pasted text(24)

This principle visibly evolved across versions. The earlier row used strings such as `proposer -> critic`; a later design used explicit edge IDs, typed ports, and data-mapping references. Pasted text(21) Pasted text(24)

**Especially relevant to Handbook:** the developed design separates **declared** relationships such as `equivalent_to` and `supersedes` from **generated** relationships such as `depends_on`, `depended_on_by`, `reads`, `writes`, `emits`, and `routes_to`. Indexes are generated from the authoritative items rather than maintained as independent truth. Pasted text(24) Pasted text(24)

**User versus assistant:** retrieval records you defining the rebuild/extraction problem, adopting selected schema tightenings, and later accepting P0 validator fixes. The initial catalog expansion and detailed row proposals were assistant contributions. An assistant subsequently reported a hardened bundle and ten passing tests; that report is not independently verified here.

**Limit:** this is a graph registry/compiler and execution-model design, not automatically an immutable versioned evidence store. Its version fields describe behavior/contracts and compatibility; they do not establish immutable per-edge revision semantics. Pasted text(24)

---

### G. `memsrc` / Memory-as-Source  
**May 31–June 2, 2026 — direct retrieved user excerpt plus the complete original POC specification**

This is the best-corroborated user-originated idea in the recovered material.

Your **May 31, 13:10 UTC** message included:

> “write the actual memories in code so they could be parsed and indexed by any preexisting ast program.”

Retrieval records you accepting a bounded POC, requesting the exact crate/schema/specification on June 2, and later reporting hardening with adversarial fixtures and an **87-record evaluation corpus**. That last point is **your historical implementation report**, not something I verified against code.

The actual artifact is:

**`memsrc_memory_as_source_poc_spec.md` — “`memsrc` POC Crate Spec: Memory-as-Source.”**

Its documented pipeline is:

```text
memory/**/*.mem.ts
    → static AST extraction
    → typed Rust intermediate representation
    → record/graph validation
    → deterministic .memsrc artifacts
```

Outputs include `records.jsonl`, `graph.json`, `chunks.jsonl`, `symbols.json`, diagnostics, and schema data. Memory source is never executed. memsrc_memory_as_source_poc_spec memsrc_memory_as_source_poc_spec

**What you originated:** representing actual memories in programming-language source to reuse AST/indexing machinery.

**What the assistant concretized:** constrained TypeScript, imported symbols as references, Rust extraction, the schema, artifact formats, and the detailed POC design.

The document makes identity separation explicit:

> “The exported const symbol is the code handle.  
> The string id is the graph handle.  
> Both should be stable.” memsrc_memory_as_source_poc_spec

Records include claims, typed relations, evidence references, and lifecycle information. Evidence can include a locator, quote, and hash; lifecycle includes supersession, retraction, and validity periods. Extracted data retains source locations, source hashes, and extractor-profile versions. memsrc_memory_as_source_poc_spec memsrc_memory_as_source_poc_spec

**Two boundaries are important:**

First, the inspected edge schema does **not** contain an independent edge ID or edge revision. Stable memory IDs and lifecycle links are therefore not proof of a fully versioned-edge model. memsrc_memory_as_source_poc_spec

Second, the specification describes walking and parsing source files, resolving references, building the graph, and emitting artifacts. It does not specify dependency-local incremental rebuilding. SQLite and filesystem watching appear only as optional later additions. memsrc_memory_as_source_poc_spec

The design was also deliberately falsifiable: compare against equivalent YAML, and reject the POC direction if TypeScript adds complexity without meaningful AST/navigation benefits. That is a particularly strong historical example of your current preference to **prove the simpler foundation rather than assume the elaborate one is justified**—although the exact failure criteria are specification text, not all direct user quotations. memsrc_memory_as_source_poc_spec

---

### H. Handbook: immutable Snapshot Memory and evidence-store antecedents  
**Undated standalone HCM-4.6 document; separately dated September 27, 2026 audit context — original documents**

The recovered `README(1).md` is titled:

**“HCM-4.6 Production Snapshot Grounding Owner and Ordinary SDK Seam.”**

It describes a repository rooted at:

```text
.handbook/state/snapshot-memory/v1/
```

Repository, stream, and boundary identities bind exact policy and authority observations. Ordering comes from Engine-assigned sequence—not filenames, directory order, or caller timestamps. README(1)

The central storage statement is directly relevant:

> “Snapshots, boundary records, source accounting, deltas when retained, and owner receipts are immutable content-addressed records.”

Mutable heads and indexes are described as **derived, bounded, and rebuildable**. Writes use a repository-local journal and atomic replacement; recovery completes or rolls back prepared transactions and rebuilds indexes. README(1)

The document also contains a **retention graph**: exact holds arise from open sessions, applicable predecessors, and retained derived records. Deletion requires proving safe reachability and hold state; uncertainty retains the bytes. Compaction cannot rewrite canonical record identities. README(1)

This is stronger historical evidence for file-backed graph-like persistence and recovery than the earlier brainstorming. However, it also requires owner locking and currentness checks around capture; it does **not** say content addressing eliminates all physical or authority revalidation. README(1)

**Status caveat:** the document calls itself an implemented checkpoint, but explicitly places implementation truth in source and tests. I verified the document, not the implementation. README(1) README(1)

Separately, the **September 27 HCM comprehensive semantic audit packet** includes HCM-5.2 evidence-store context and distinguishes occurrence/content identity, evidence closure, exact replay, and present-use reliance. Its own introduction warns that implementation claims are documentary evidence, not fresh verification. This is independently recovered older Handbook material—not a reconstruction from today’s prompt. hcm-final-semantic-audit-packet hcm-final-semantic-audit-packet

---

### I. Unified Agent API support-depth: exact dependency bindings and selective invalidation  
**September 28–29 proposal material; October 2-dated draft preserved in an October 10 review source — original documents**

**This is the most directly relevant prior concept for “verify the exact dependencies, not every unrelated historical file.”**

The September research/decision material recommends:

- An **acyclic content-addressed design** using SHA-256 and explicitly versioned canonical JSON.
- Human-authored TOML where appropriate, without treating formatting changes as semantic changes.
- References between payloads rather than circular embedding or hashes of their own future outputs.
- Separate policy, obligation, and evolving evidence identities. Pasted text(20260929-023608)

Its dependency-binding proposal is unusually concrete:

> “Sorted repo-relative leaf paths/content hashes, referenced protocol/fixture identities and resolved build graph; root hash over named leaves.” Pasted text(20260929-023608)

It then recommends beginning with the affected crate’s transitive build-dependency closure plus referenced adapters, protocols, tests, and fixtures—not hand-picked function hashes, but also **not the whole repository commit**. Missing dependency information is unverified, not an empty dependency set. Pasted text(20260929-023608)

The later **Support-Depth Contract** draft states:

> “Missing dependency provenance is `unverified`, not an empty set. An unrelated change invalidates nothing.”

Reuse must establish unchanged requirements, trust/review rules, applicable version and target, and every input in the obligation’s dependency set. repo-review-source-001(20261010…

**Attribution and status:** the detailed canonicalization/dependency design entered through advisory research. The user-supplied wrapper preserves your instructions about reproducible incremental maintenance, exact identities, and avoiding unnecessary greenfield migration machinery. That does not make every recommended schema choice your originating idea. Pasted text(20260929-023608)

The later contract remains explicitly:

> “Status: Draft, awaiting maintainer approval”

and says it binds nothing while Draft. It is therefore historical design evidence, **not proof of adopted or implemented enforcement**. repo-review-source-001(20261010…

**A real conversation link was recovered here:** the earlier document records a [ChatGPT Pro support-depth consultation](https://chatgpt.com/c/6ab9a464-4684-83ea-8391-ea88e04e8849). I recovered that link from the document; I did not open the complete linked conversation. The document itself identifies it as advice, not executed repository verification. Pasted text(20260929-023608)

## 3. Recurring principles, evolution, and genuine disagreements

### Files as authoritative material; graphs and indexes as computed views

The strongest recurring pattern is **not “graphs without software infrastructure.”** It is:

**Keep inspectable source material authoritative; derive machine-oriented graph and retrieval structures from it.**

That appears as YAML units versus passports, TypeScript memory declarations versus `.memsrc` artifacts, C3 items versus generated indexes, and HCM committed records versus rebuildable heads/indexes. The May file-canonical conversation makes the same distinction at the conceptual level. substrate_high_level_technical\_… memsrc_memory_as_source_poc_spec Pasted text(24) README(1)

**My synthesis:** the recurring preference is for clear authority and recoverability, not a universal prohibition on databases. An optional database could be compatible with some earlier designs; making it indispensable canonical truth would be a different choice.

### Identity became more explicit—but several kinds of versioning remain distinct

The history contains multiple identity mechanisms:

| Mechanism | Recovered examples | What it does not automatically provide |
|---|---|---|
| File/repository history | Spec-only Git history | Immutable identity for each graph assertion |
| Stable semantic IDs | Semantic units, memory IDs, C3 item/edge IDs | Exact content-version identity |
| Lifecycle/supersession | `memsrc` supersedes/retraction; C3 declared supersession | Transactional immutable edge revisions |
| Content-bound records | HCM snapshots; support-depth payloads and dependency leaves | A complete general-purpose Merkle-DAG protocol |

The explicit distinction between symbol handles and graph IDs in `memsrc`, and between immutable records and mutable heads in HCM, is especially valuable. It would be inaccurate to combine all these mechanisms retrospectively and say you already had a fully specified versioned graph store. memsrc_memory_as_source_poc_spec README(1)

### Relationships increasingly became typed, checkable objects

C3 shows the clearest visible refinement: topology represented as prose-like strings gave way to explicit edge IDs, port contracts, referenced mappings, and lintable relationships. Its later distinction between authored and generated relationships also avoids maintaining the same fact independently in multiple places. Pasted text(21) Pasted text(24) Pasted text(24)

Not every recovered graph is a DAG. C3 explicitly models bounded execution loops, while support-depth recommends acyclic content-addressed bindings. Those solve different problems and should not be collapsed into one universal topology rule. Pasted text(24) Pasted text(20260929-023608)

### Rebuildability is well evidenced; fine-grained incremental maintenance is less complete

Several designs can regenerate derived artifacts. That is not the same as proving selective invalidation.

`memsrc` provides source hashes and deterministic extraction, but the documented extraction path still walks and processes files. The March semantic-unit design places incremental detection later. Support-depth is where the recovered material most clearly requires **dependency-specific reuse and invalidation**, including a conservative completeness floor. memsrc_memory_as_source_poc_spec substrate_high_level_technical\_… Pasted text(20260929-023608)

**My inference:** your history supports investigating incremental verification, but does not justify claiming that an existing graph cache already proves which unrelated history can safely be skipped.

### Provenance and authority are consistently separate from convenient representations

The March architecture distinguishes declared test coverage from observed execution. `memsrc` distinguishes user-stated, observed, inferred, reasoned, and derived claims. C3 requires source and behavioral proof anchors. Support-depth treats an evidence binding as a claim submitted for checking, not a certification flag. substrate_high_level_technical\_… memsrc_memory_as_source_poc_spec Pasted text(24) Pasted text(20260929-023608)

This is a particularly strong recurring principle: **an index can locate a claim; it does not thereby establish the claim’s validity or authority.**

The later support-depth language likewise says its four bindings are relationships, not a requirement for four files, stores, or schemas. That is relevant to avoiding redundant physical machinery while preserving distinct invalidation and authority semantics. repo-review-source-001(20261010…

### File-backed integrity grew into explicit recovery semantics

HCM adds details absent from the earlier memory concepts: transaction journals, replay, exact predecessors, owner locks, durability uncertainty, retention reachability, and non-rewriting compaction. These are genuine file-backed persistence concerns already present in your historical documents. README(1)

However, their presence also argues against casually classifying every repeated filesystem check as redundant. Content identity, current authority, physical repository binding, and durable publication are distinct questions in that design.

### What was corrected, deferred, or potentially rejected

The evidence supports several real distinctions, but fewer firm “abandoned architecture” claims:

**Explicit correction:** you rejected the assistant’s interpretation of putting Git inside Spec and clarified that you wanted to fork Git.

**Visible draft evolution:** C3 moved away from opaque topology strings toward referenced, checkable objects. Earlier shapes should not be treated as equally current.

**Conditional rejection, not proven abandonment:** `memsrc` defined reasons to abandon the TypeScript approach if it failed to outperform YAML. I found a later user report of hardening, not evidence that the experiment was abandoned. memsrc_memory_as_source_poc_spec

**Alternatives without a recovered final decision:** Postgres, LadybugDB, embedded graph stores, and direct JSON/JSONL graphs were discussed in different branches. I did not recover a blanket historical rejection of SQLite or all databases.

**Explicit simplification pressure:** your support-depth clarification opposed migration/legacy machinery without an architectural purpose while preserving useful versioning and exact evidence identities. That is a rejection of unnecessary machinery, not of versioned data itself. Pasted text(20260929-023608)

## 4. Prior concepts worth revisiting for Handbook

The following are **present relevance judgments about recovered concepts**, not claims that you previously assembled this exact Handbook design.

| Prior concept | Why it is relevant now | Historical boundary to preserve |
|---|---|---|
| **Support-depth’s content-bound dependency closure** | Closest prior answer to verifying the inputs relevant to one operation while leaving unrelated changes alone. | The dependency set must be complete enough for the claim; smaller hand-selected sets were not automatically trusted. |
| **HCM immutable records with rebuildable heads/indexes** | Existing documentary foundation for file-backed identity, recovery, replay, and retention. | Rebuildable indexes do not replace authority/currentness or durable-commit checks. |
| **C3 authoritative items plus generated relationships** | Useful precedent for avoiding duplicate manually maintained dependency and reverse-dependency truth. | Declared semantics and generated relationships must remain distinguishable and checkable. |
| **Lift’s snapshot-provider boundary** | Most relevant recovered concept for avoiding repeated downstream filesystem acquisition. | This is an assistant-proposed boundary recovered through summaries; its shipped guarantees remain unverified. |
| **May’s direct file relations and the `memsrc` comparison discipline** | Precedent for establishing useful graph behavior over files before making another storage mechanism necessary. | Source-language sophistication or a database must earn its role; neither was universally adopted. |

The first three are supported by original documents, including explicit dependency rules, recovery semantics, and generated-index requirements. Pasted text(20260929-023608) README(1) Pasted text(24)

**What I did not recover:** a single original, approved design that combines immutable versioned nodes, independently versioned edges, complete dependency discovery, incremental reverse-index maintenance, dependency-local verification, and crash-safe publication into one finished file-based graph implementation.

That combination would be a new integration decision. Its individual ingredients have meaningful historical antecedents.

## 5. Unresolved questions

These are the gaps that remain after retrieval, rather than a new architecture prescription.

| Question | Why the recovered history does not settle it |
|---|---|
| **What exactly would Handbook’s graph reference: a logical object, an immutable revision, an occurrence, or a current head?** | The historical designs use all of these distinctions in different places; they are not interchangeable. |
| **Are edges independent immutable records, or facts embedded in versioned nodes?** | C3 has edge IDs; `memsrc` has embedded edge records without independent revision identity. Neither establishes the proposed Handbook choice. |
| **Who proves that an operation’s dependency set is complete?** | Support-depth explicitly rejects missing provenance and overly narrow hand-picked hashes, but that does not supply Handbook’s dependency-discovery protocol. |
| **Which changes invalidate historical content, and which only invalidate present reliance on it?** | HCM separates historical results from present-use/currentness decisions. A graph cache must not silently erase that distinction. |
| **Which filesystem checks establish new facts, and which repeat a fact already guaranteed by a valid owner/snapshot boundary?** | Lift suggests a way to concentrate acquisition; HCM preserves boundary-specific rechecks. The actual checks and guarantees in current Handbook were not inspected here. |
| **How are index freshness, interrupted updates, and deletion/recovery proved without scanning unrelated history on every operation?** | HCM provides recovery and reachability principles; the retrieved material does not complete a dependency-local maintenance protocol for the proposed graph. |

**Bottom line:** your history contains a clear recurring direction toward **inspectable file-based truth, explicit graph relationships, derived/rebuildable indexes, and evidence tied to exact inputs**. The strongest material to revisit is the **May file-canonical concept, `memsrc`, C3, HCM Snapshot Memory, and support-depth’s dependency-closure rules**. What remains unproven is not whether you previously had relevant ideas—you did—but whether those ideas already supply the exact identity, completeness, and recovery guarantees needed to safely eliminate particular checks in today’s Handbook.
