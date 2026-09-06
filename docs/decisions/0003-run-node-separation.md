# ADR-0003: Run and Node Separation

- Status: Baseline
- Date: 2026-09-06

## Context

AI conversation generation involves asynchronous, long-running, streaming, and failure-prone operations.

A model generation may fail halfway, be cancelled by the user, time out, or produce invalid tool call payloads.

If transient execution states are directly modeled as conversation `Node`s, the conversation history graph becomes polluted with dirty, half-written, or failed states.

GitLLM requires a clean distinction between transient execution processes and persistent conversation facts.

## Decision

The system strictly separates `Run` from `Node`.

- A `Run` represents an execution lifecycle (state: Pending, Running, Completed, Interrupted, Failed, Cancelled). It manages streaming buffers, retries, and execution metadata.
- A `Node` represents an immutable historical fact persisted into the conversation graph only after generation successfully completes or reaches an accepted checkpoint.

Transient failures or cancellations terminate the `Run` without leaving garbage `Node`s in the history graph.

## Consequences

### Benefits

- The conversation graph contains only validated, immutable historical states.
- Cancellation and failure logic do not require rollbacks or deletions on the immutable graph.
- Clean architectural boundary between execution runtime and domain core.

### Costs

- The application layer must coordinate the lifecycle of `Run` and explicitly convert completed runs into `Node`s.
- State machines for `Run` add architectural surface to the runtime crate.

## Alternatives Considered

### Node-only Model with Dirty Flags

Create a `Node` immediately in "draft" or "generating" status and mutate it in place.
_Rejected_: Violates ADR-0001 (Node Immutability) and complicates graph queries.

## Related Decisions

- ADR-0001: Node Immutability
- ADR-0002: Branch as Reference
