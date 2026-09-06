---

# 最后是 `docs/roadmap.md`

这个文件只描述**方向**：

```markdown
# GitLLM Roadmap

## Core Development

### M0 — Bootstrap

- Rust workspace
- Core crate
- Documentation baseline
- ADR structure
- Git/GitHub workflow
- CI
- Windows/Linux validation

### M1 — Core Domain

- Strongly typed identifiers
- Conversation
- Node
- MessagePayload
- ContentPart
- Branch
- Run
- basic serialization
- domain unit tests

### M2 — Conversation Graph

- parent relationships
- graph traversal
- ancestor queries
- LCA
- history validation
- reference relationships

### M3 — Branch Operations

- create branch
- fork
- checkout
- branch advancement
- edit-as-fork semantics
- branch state validation

### M4 — Snapshot / Commit / Diff

- Snapshot
- Commit
- structural diff
- content diff
- restore/reset semantics

### M5 — Context Engine

- context selection
- context policies
- token budgeting
- context overflow handling
- synthesis/distill barriers

### M6 — Search and Import/Export

- conversation search
- JSON import/export
- Markdown import/export
- provenance preservation

### M7 — Repository and Robustness

- repository abstraction
- persistence-oriented interfaces
- crash recovery
- concurrency tests
- stronger invariants

### M8 — Core v0.1

- stable Core API
- comprehensive tests
- documentation
- cross-platform validation

## Future Product Layers

After the Core milestone:

- Runtime
- Provider integrations
- PostgreSQL
- HTTP API
- Web UI
- Authentication
- Collaboration
- BYOK
- External source adapters
- Agent ledger
- MCP
- parallel exploration
- review
- synthesis
- advanced distillation

## Roadmap Policy

This roadmap is directional rather than contractual.

Architectural decisions that materially change the current baseline should be recorded through ADRs.

Implementation order may change as engineering experience reveals better approaches.
```
