# ADR-0006: Edit Implies Implicit Fork

- Status: Baseline
- Date: 2026-09-06

## Context

In mainstream AI chat interfaces, editing a previous user message either overwrites the message or truncates subsequent turns, creating a confusing historical trail.

In GitLLM, historical conversation integrity is foundational. Users need the ability to refine a past prompt or restart reasoning from a previous assistant answer without destroying the downstream conversation that already occurred.

## Decision

Any "Edit" operation on an existing historical `Node` is semantically defined as an **Implicit Fork**.

When a user or agent edits Node $N_k$:

1. Original Node $N_k$ remains unchanged (ADR-0001).
2. The system creates a new evolution branch starting from $N_k$'s parent ($N_{k-1}$).
3. A new Node $N'_k$ is created with the edited payload and points to $N_{k-1}$ as its `primary_parent`.
4. The active branch pointer switches to this newly created branch.

This rule applies universally to both User prompts and Assistant responses (e.g., Regeneration).

## Consequences

### Benefits

- Zero risk of accidental history loss.
- Perfectly aligned with user mental models: "I want to try asking this differently without losing my current work".
- Enables side-by-side comparison (Diff) between original and edited evolution branches.

### Costs

- UI/CLI must clearly show when an implicit fork has occurred to prevent user disorientation.
- Branch naming or indexing strategies must handle automated fork proliferation gracefully.

## Alternatives Considered

### In-place History Truncation (Mainstream Chat UI Model)

Delete everything after $N_k$ and overwrite $N_k$.
_Rejected_: Destroys user exploration history and breaks referential integrity.

## Related Decisions

- ADR-0001: Node Immutability
- ADR-0002: Branch as Reference
