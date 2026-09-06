# ADR-0001: Node Immutability

- Status: Baseline
- Date: 2026-09-06

## Context

GitLLM needs to preserve the history of AI conversations while allowing users to explore alternative reasoning paths.

If historical conversation nodes can be modified in place, the meaning of existing branches, snapshots, commits, diffs, and provenance becomes unstable.

A user editing an old message should not silently rewrite the history that other branches or users may still depend on.

GitLLM therefore needs a stable unit of historical state.

## Decision

A `Node` is immutable by default.

Normal business operations must not mutate an existing Node after it has been persisted as conversation history.

When a user changes an existing historical node, the system creates a new evolution path instead of modifying the original node.

Logical redaction may replace visible content with a redaction marker when required by privacy or compliance operations.

Physical deletion is a separate operation and is not considered a normal conversation-history mutation.

## Consequences

### Benefits

- Historical conversation state remains stable.
- Branches can safely share existing history.
- Snapshots and commits remain meaningful.
- Diff and provenance become easier to reason about.
- Concurrent users do not silently rewrite each other's history.
- The design aligns with GitLLM's versioned-history philosophy.

### Costs

- Editing history creates additional Nodes.
- Storage usage increases over time.
- APIs and UI must distinguish historical state from new evolution paths.

### Constraints

Any future feature that appears to require in-place Node mutation must either:

1. operate outside historical Node state, or
2. introduce a new architectural decision through an ADR.

## Alternatives Considered

### Mutable Nodes

Rejected because mutating historical Nodes would weaken branch consistency, provenance, and reproducibility.

### Copying the Entire Conversation on Edit

Rejected because it introduces unnecessary duplication.

GitLLM should reuse existing immutable history and create only the new evolution path.

### Hard Delete as Normal Mutation

Rejected because deletion is fundamentally different from normal conversation evolution and may be governed by compliance or privacy requirements.

## Related Decisions

- ADR-0002: Branch as Reference
- ADR-0006: Edit Implies Fork
