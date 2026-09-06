# GitLLM Overview

## What is GitLLM?

GitLLM is a unified, versioned, branchable, and explorable workspace for AI conversations.

It provides a structured way to preserve, branch, compare, review, and distill AI conversation history across different models, agents, and external sources.

GitLLM is inspired by Git, but it is not intended to be a literal implementation of Git for LLMs.

## Core Ideas

GitLLM is built around four core capabilities:

- Unify — bring conversations from different sources into one canonical model.
- Fork — create independent exploration paths from an existing conversation state.
- Explore — use different models, providers, or external agents on independent branches.
- Distill — compress useful results from deep exploration into a compact context for continued work.

Core product principle:

> Fork wide, distill narrow.

## Core Domain Concepts

The initial domain model contains:

- Conversation
- Branch
- Node
- Snapshot
- Commit
- Run
- Provider
- Model
- Credential
- Source
- NodeReference

Later versions may introduce:

- ReviewRun
- Task
- TaskDependency
- Synthesis
- Agent execution metadata

## Design Principles

### Conversation-first

GitLLM is designed around AI conversation semantics rather than reproducing Git internally.

### Immutable history

Historical Nodes are immutable by default. New changes create new evolution paths instead of mutating existing history.

### Graph for history, context for execution

The conversation graph records relationships and history.

The Context Engine determines what information is actually presented to an LLM.

### Git-inspired, not Git-constrained

GitLLM adopts useful concepts such as branches, snapshots, commits, and diffs while allowing different semantics where AI conversations require them.

### Core-first

The domain core should remain independent from UI, storage engines, web frameworks, MCP, and external infrastructure.

## Current Development Stage

The project is currently in the M0 bootstrap phase.

The current implementation goal is to establish the cross-platform Rust core and its engineering foundations before implementing runtime, storage, API, UI, and agent integrations.
