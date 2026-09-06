# GitLLM Architecture

## Current Architecture

GitLLM is organized into several conceptual layers:

```text
UI / CLI / MCP / External Agent / Import
                  ↓
          Application Layer
                  ↓
           GitLLM Runtime
                  ↓
             GitLLM Core
                  ↓
          Repository Trait
                  ↓
       InMemory / PostgreSQL
```

The current implementation begins with the Core layer.

Core

The Core layer contains domain concepts and domain rules.

It must not depend directly on:

Axum
PostgreSQL
SQLx
MCP
UI frameworks
operating-system-specific APIs
network clients

The initial Core implementation uses in-memory structures where persistence is required for testing.

Runtime

The Runtime layer will eventually coordinate:

model execution
streaming
provider selection
concurrency
cancellation
retries
execution lifecycle

Runtime is intentionally outside the Core.

Storage

Persistence will be introduced behind repository abstractions.

The initial production database is planned to be PostgreSQL, but Core must remain storage-agnostic.

API

The API layer will later expose GitLLM functionality through HTTP and related protocols.

Expected future technologies include:

Axum
REST APIs
SSE
MCP

These technologies are not part of the current Core implementation.

External Sources

External conversation sources will be adapted into GitLLM's canonical conversation model.

Potential sources include:

web-based AI products
official model APIs
local agents
AI IDEs
imported files

External source integration is intentionally separated from the Core domain model.

Agents

GitLLM is not intended to become a full agent runtime.

External agents may act as branch executors or ledger clients.

GitLLM primarily manages:

branch state
execution history
provenance
context
distillation
Future Expansion

Future architecture may include:

PostgreSQL persistence
provider abstraction
web UI
CLI
MCP integration
collaboration
parallel exploration
review
synthesis
agent integration
