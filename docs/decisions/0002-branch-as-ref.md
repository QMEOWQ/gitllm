# ADR-0002: Branch as Reference

- Status: Baseline
- Date: 2026-09-06

## Context

A conversation may contain many branches that share a large amount of history.

Representing each branch as a copied list or tree of all Nodes would duplicate state and increase the cost of branching.

GitLLM needs branching to be cheap and to preserve shared history naturally.

## Decision

A Branch is a reference to a conversation history state.

The core Branch model is:

```text
Branch
├── id
├── conversation_id
├── name
├── head
└── version
```

The head points to the current Node at the tip of the branch.

Forking a branch therefore creates a new Branch that initially points to an existing Node.

The branch does not own or duplicate the complete history.

History is reconstructed through Node relationships.

Consequences
Benefits
Forking is conceptually O(1).
Large histories are naturally shared.
Branches remain lightweight.
History can be represented without copying entire conversation trees.
Concurrent branch exploration becomes easier.
Costs
History traversal requires following Node relationships.
Queries over long histories may require indexes or caching later.
Branch state and Node history are separate concepts and must not be conflated.
Constraints

The initial implementation must not maintain a per-branch authoritative array containing every Node ID.

Derived caches may be introduced later when measurements justify them.

Alternatives Considered
Copy the Entire Conversation

Rejected because it duplicates history and makes branch creation unnecessarily expensive.

Store Every Node ID on Each Branch

Rejected because it creates a synchronization and consistency burden.

Global Mutable Current Branch

Rejected because multiple users may view or work on different branches of the same conversation concurrently.

Related Decisions
ADR-0001: Node Immutability
ADR-0003: Run and Node Separation
ADR-0004: Optimistic Concurrency Control
