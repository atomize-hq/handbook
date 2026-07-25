# Migrating to handbook-engine 0.2.0

`handbook-engine` 0.2.0 records the source-compatibility boundary introduced by
the HCM-2.3 publication contract. Two public enums gained one variant each:

| Rust variant | Serialized value |
|---|---|
| `EstablishedRefusalCodeV1::PublicationBasisConflict` | `publication_basis_conflict` |
| `GenericRefusalLayerV1::Publication` | `publication` |

Consumers must update exhaustive Rust matches to handle both variants and must
admit both exact snake-case values in downstream deserializers, validators, and
allowlists. No aliases are accepted.

`EstablishedRefusalV1` and its fields are unchanged, so existing serialized
records do not require rewriting. The new values represent a terminal
publication-basis conflict at the final native-publication boundary; they are
not valid authority for a pre-publication planned refusal.

Workspace consumers with an explicit dependency constraint should use:

```toml
handbook-engine = "0.2.0"
```

This repository change establishes the compatibility boundary only. It does not
tag or publish `handbook-engine` 0.2.0.
