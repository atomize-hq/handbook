# Engineering Charter — Boundary Project

## What this is
This engineering charter is the canonical decision surface for Boundary Project. It turns the project's stated baseline posture, domain constraints, and dimension-specific guardrails into the default rules for day-to-day engineering work.

## How to use this charter
- Default to the project baseline of level 3 (Product) unless a dimension or domain section explicitly raises or lowers the bar.
- Use the dimension sections below to decide when to raise rigor, what shortcuts remain acceptable, and which red lines are non-negotiable.
- Record approved exceptions in .handbook/decisions/exceptions before deviating from these defaults.
- Revisit this charter when the project classification, risk profile, or operating environment changes.

## Rubric: 1–5 rigor levels
| Level | Label | Meaning |
|------:|-------|---------|
| 1 | Exploratory | throwaway ok; optimize learning; minimal gates |
| 2 | Prototype | demoable/internal use; some structure; still speed-first |
| 3 | Product | real users; balanced; maintainability matters |
| 4 | Production | GA/customer-facing; strong quality/reliability/security defaults |
| 5 | Hardened | critical/regulated/high blast radius; strict gates; defense-in-depth |

## Project baseline posture
- **Baseline level:** 3 (Product)
- **Baseline rationale:**
  - Real users and durable maintenance require balanced rigor
- **Project classification:** greenfield
- **Users:** mixed
- **Expected lifetime:** years
- **Team size:** 3
- **Surfaces:** web app, api
- **Runtime environments:** browser, server
- **Deadline:** 2026 Q4
- **Budget notes:** Small team
- **Experience notes:** One senior and two early-career engineers
- **Required technologies:** none declared
- **In production today:** no
- **Production users or data:** No production users or data
- **External contracts to preserve:** none declared
- **Uptime expectations:** Business-hours support
- **Backward compatibility default:** boundary only
- **Migration planning default:** required
- **Rollout controls default:** lightweight
- **Deprecation policy default:** not required yet
- **Observability threshold default:** standard

## Domains / areas (optional overrides)
These domains add context for where the baseline posture needs extra care.

### Identity
- **Blast radius:** Account access and session trust
- **Touches / trust boundary:** Authentication boundary
- **Special constraints:** Never log credentials
- **Default posture:** baseline applies unless a dimension override below says otherwise.

## Posture at a glance (quick scan)
| Dimension | Default level (1–5) | Notes / intent |
|---|---:|---|
| Speed vs Quality | 3 | Prefer small reversible changes |
| Type safety / static analysis | 3 | Use strict compiler and lint settings |
| Testing rigor | 4 | Test behavior at the owning boundary |
| Scalability & performance | 3 | Measure before optimizing |
| Reliability & operability | 4 | Fail closed at authority boundaries |
| Security & privacy | 4 | Minimize sensitive data and trust |
| Observability | 3 | Emit typed bounded diagnostics |
| Developer experience (DX) | 3 | Automate repeatable proof |
| UX polish / API usability | 3 | Keep interfaces explicit and actionable |

## Dimensions (details + guardrails)
Each dimension inherits the project baseline unless an explicit level is set below.

### 1) Speed vs Quality
- **Default stance (level):** 3 (Product)
- **Intent:** Prefer small reversible changes
**Raise the bar when:**
- Irreversible data change

**Allowed shortcuts when:**
- Time-boxed prototype behind a flag

**Non-negotiables / red lines:**
- Do not bypass required review

**Domain overrides (if any):**
- None — baseline applies.

### 2) Type safety / static analysis
- **Default stance (level):** 3 (Product)
- **Intent:** Use strict compiler and lint settings
**Raise the bar when:**
- Public typed interface

**Allowed shortcuts when:**
- Localized parsing adapter

**Non-negotiables / red lines:**
- Do not suppress unknown unsafe behavior

**Domain overrides (if any):**
- None — baseline applies.

### 3) Testing rigor
- **Default stance (level):** 4 (Production)
- **Intent:** Test behavior at the owning boundary
**Raise the bar when:**
- Constitutional authority transition

**Allowed shortcuts when:**
- Documented non-production spike

**Non-negotiables / red lines:**
- Do not ship an untested authority transition

**Domain overrides (if any):**
- Identity remains level 4

### 4) Scalability & performance
- **Default stance (level):** 3 (Product)
- **Intent:** Measure before optimizing
**Raise the bar when:**
- Budget threshold exceeded

**Allowed shortcuts when:**
- Bounded fixture data

**Non-negotiables / red lines:**
- Do not hide unbounded work

**Domain overrides (if any):**
- None — baseline applies.

### 5) Reliability & operability
- **Default stance (level):** 4 (Production)
- **Intent:** Fail closed at authority boundaries
**Raise the bar when:**
- Production write path

**Allowed shortcuts when:**
- Read-only local experiment

**Non-negotiables / red lines:**
- Do not report partial success

**Domain overrides (if any):**
- Identity requires rollback proof

### 6) Security & privacy
- **Default stance (level):** 4 (Production)
- **Intent:** Minimize sensitive data and trust
**Raise the bar when:**
- Credential or personal-data handling

**Allowed shortcuts when:**
- Synthetic data only

**Non-negotiables / red lines:**
- Do not persist secrets

**Domain overrides (if any):**
- Identity requires threat review

### 7) Observability
- **Default stance (level):** 3 (Product)
- **Intent:** Emit typed bounded diagnostics
**Raise the bar when:**
- Unattended production operation

**Allowed shortcuts when:**
- Local deterministic test

**Non-negotiables / red lines:**
- Do not log sensitive values

**Domain overrides (if any):**
- None — baseline applies.

### 8) Developer experience (DX)
- **Default stance (level):** 3 (Product)
- **Intent:** Automate repeatable proof
**Raise the bar when:**
- Repeated manual release step

**Allowed shortcuts when:**
- One-time documented command

**Non-negotiables / red lines:**
- Do not automate hidden authority

**Domain overrides (if any):**
- None — baseline applies.

### 9) UX polish / API usability
- **Default stance (level):** 3 (Product)
- **Intent:** Keep interfaces explicit and actionable
**Raise the bar when:**
- Public or irreversible operation

**Allowed shortcuts when:**
- Internal diagnostic wording

**Non-negotiables / red lines:**
- Do not hide refusal reasons

**Domain overrides (if any):**
- None — baseline applies.

## Cross-cutting red lines (global non-negotiables)
- No silent constitutional default
- No agent self-approval

## Constitutional policy and governance
- **Policy revision:** 1
- **Authority statement:** This Charter is the approved project engineering authority
- **Decision authority:** Project owner
- **Required approvals:** Project owner approval
- **Exception policy:** Exceptions require an approved record before implementation
- **Review triggers:** Material policy change
- **Reassessment triggers:** Production posture changed, Trust boundary changed

## Exceptions / overrides process
- **Approvers:** Project owner
- **Record location:** .handbook/decisions/exceptions
- **Minimum required fields:**
  - Scope
  - Rationale
  - Expiry

## Debt tracking expectations
- **Tracking system:** Repository issue tracker
- **Labels:** technical-debt
- **Review cadence:** monthly

## Decision Records (ADRs): how to use this charter
- Record major design decisions in .handbook/decisions using Markdown ADR files.
- Use ADRs when a change alters the project baseline, crosses a listed red line, or introduces a lasting domain override.

## Review & updates
- Review this charter on a monthly cadence.
- Update it when the project classification, domains, runtime environments, or production reality change.
- Re-run impacted plans when any update changes a default level, a red line, or an exception process.
