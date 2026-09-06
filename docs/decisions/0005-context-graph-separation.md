# ADR-0005: Separation of History Graph and Execution Context

- Status: Baseline
- Date: 2026-09-06

## Context

A conversation graph records the full relational history: user inputs, agent branches, tool call traces, distillation summaries, and cross-branch references.

However, an LLM prompt cannot simply be a topological dump of the conversation graph due to:

1. Finite model context windows (token limits).
2. Cost and latency constraints.
3. Information noise from failed exploration branches.
4. Provider-specific message format differences.

## Decision

The relational `Graph` is decoupled from the runtime `Context`.

- The `Graph` records objective historical relationships and provenance.
- The `Context Engine` acts as a materialization pipeline that selects, filters, budgets, and transforms graph nodes into a linear model prompt.

The pipeline comprises:

1. `ContextSelector`: Selects candidate nodes along the branch ancestry and referenced paths.
2. `ContextBudgeter`: Evaluates token usage against model constraints.
3. `ContextMaterializer`: Serializes nodes into model-specific formats, injecting distillation boundaries and summaries where deep history is pruned.

## Consequences

### Benefits

- Enables advanced capabilities like distillation barriers without altering graph history.
- Domain Core remains decoupled from LLM provider context formats.
- Supports flexible context selection strategies (sliding window, pin-and-compress, distillation back-channel).

### Costs

- Context materialization adds a computational layer between storage read and LLM invocation.

## Alternatives Considered

### Direct Linear History Traversal

Simply traverse `primary_parent` backwards to root and format as prompt.
_Rejected_: Inadequate for branch distillation, cross-branch references, or large context management.

## Related Decisions

- ADR-0001: Node Immutability
- ADR-0002: Branch as Reference
