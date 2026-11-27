---
description: The framestack.toml format, version 1, and the rules framestack check applies to it.
---

# Manifest reference (framestack.toml, version 1)

A FrameStack project is described by one `framestack.toml` file at its root. It lists the project's components and the relationships between them, following the project graph in [ADR-0005](../decisions/0005-project-graph.md).

`framestack check` validates the file in two passes:

1. **Structure:** TOML syntax, required fields, value types, and unknown fields. Unknown fields in core tables are always rejected, so a misspelled key fails instead of being ignored ([ADR-0008](../decisions/0008-manifest-and-tooling.md)).
2. **Meaning:** references between components, cycles, capabilities, paths and names. These rules are listed under [Validation rules](#validation-rules).

## Example

```toml
manifest_version = 1

[project]
name = "typescript-reference"

[[components]]
id = "web"
kind = "application"
path = "apps/web"
language = "typescript"
integrations = ["react", "vite"]
provides = ["frontend.browser", "frontend.spa"]
requires = ["api.rest"]

[[components]]
id = "api"
kind = "service"
path = "services/api"
integrations = ["fastify"]
provides = ["http.server", "http.api", "api.rest"]
env = ["PORT", "DATABASE_URL"]

[[relationships]]
from = "web"
to = "api"
type = "communicates_with"
protocol = "http"
```

More examples, valid and invalid, are in [`fixtures/manifest/`](https://github.com/cybercoretech/framestack/tree/main/fixtures/manifest).

## Top level

| Field | Required | Meaning |
| --- | --- | --- |
| `manifest_version` | yes | Format version. Only `1` exists. |
| `project` | yes | Project table: `name` (required, not blank) and `description` (optional). |
| `components` | no | Array of component tables. |
| `relationships` | no | Array of relationship tables. |

## Components

| Field | Required | Meaning |
| --- | --- | --- |
| `id` | yes | Unique ID within the project: lowercase letters and digits, starting with a letter, with single `-` or `_` between parts. Example: `api-gateway`. |
| `kind` | yes | `application`, `service`, `worker` or `resource`. The kind is structural; the framework comes from `integrations`. |
| `path` | one of `path` or `external` | Working directory, relative to the manifest. It must stay inside the project: no absolute paths, no `..`, and forward slashes only. |
| `external` | one of `path` or `external` | Where a component FrameStack does not build lives, such as a managed database. |
| `language` | no | For example `rust` or `typescript`. |
| `runtime` | no | For example `node`. |
| `integrations` | no | Integration IDs, such as `leptos`, `axum`, `react`, `vite` or `fastify`. Same format as component IDs, and `.` is also allowed. |
| `provides` | no | Capabilities the component provides. |
| `requires` | no | Capabilities the component needs. |
| `env` | no | Names of environment variables the component reads, such as `DATABASE_URL`. Never put values or secrets in the manifest. |
| `config` | no | Settings for one integration each, as `[components.config.<integration>]` tables. The key must be listed in the component's `integrations`. The table's contents are kept as written for that integration's adapter to check. |

## Relationships

| Field | Required | Meaning |
| --- | --- | --- |
| `from` | yes | Component ID of the source. |
| `to` | yes | Component ID of the target. |
| `type` | yes | `build_dependency`, `runtime_dependency` or `communicates_with`. |
| `protocol` | no | For `communicates_with`, such as `http`. |

- `build_dependency`: `from` needs `to` built first. These must not form a cycle.
- `runtime_dependency`: `from` needs `to` running. A cycle is reported as a warning.
- `communicates_with`: `from` talks to `to`. It does not affect build order. Two-way traffic is two relationships.

## Capabilities

Capability IDs come from the vocabulary in [ADR-0006](../decisions/0006-capabilities-and-operations.md):

| Family | IDs |
| --- | --- |
| Frontend | `frontend.browser`, `frontend.spa`, `frontend.ssr`, `frontend.ssg`, `frontend.hydration` |
| HTTP | `http.server`, `http.api`, `http.middleware` |
| API | `api.rest`, `api.graphql` |
| RPC | `rpc.grpc` |
| Realtime | `realtime.websocket`, `realtime.sse` |
| Background | `worker.background` |
| Persistence | `persistence.sql`, `persistence.nosql` |
| Messaging | `messaging.publish`, `messaging.subscribe` |

A requirement is satisfied when the component provides the capability itself, or when a component it has a `runtime_dependency` or `communicates_with` relationship to provides it. A `build_dependency` never satisfies a requirement, because building something says nothing about what it does at runtime.

In this version, capabilities are declared on components. Integration descriptors that declare capabilities for you, as ADR-0006 describes, come with the first adapters.

## Validation rules

Errors make the manifest invalid and `framestack check` exits with code 1. Warnings are reported but do not fail the check.

| Code | Severity | Rule |
| --- | --- | --- |
| FS001 | error | The file is not valid TOML, a required field is missing, a value has the wrong type, or a field is unknown. |
| FS002 | error | `manifest_version` is not `1`. |
| FS003 | error | A component ID does not match the ID format. |
| FS004 | error | Two components have the same ID. |
| FS005 | error | A relationship names a component that does not exist. |
| FS006 | error | A relationship goes from a component to itself. |
| FS007 | error | Build dependencies form a cycle. The message lists it, such as `a -> b -> a`. |
| FS008 | error | A capability ID is not in the vocabulary. |
| FS009 | error | A required capability is not provided by the component or by a component it depends on or communicates with at runtime. |
| FS010 | error | A component has neither `path` nor `external`, or has both. |
| FS011 | error | A `path` is absolute, contains `..`, or uses backslashes. |
| FS012 | error | A `config` table is for an integration the component does not list. |
| FS013 | error | An `env` entry is not a valid variable name (`A-Z`, `0-9` and `_`, not starting with a digit). |
| FS014 | warning | The same relationship is declared more than once. |
| FS015 | warning | Runtime dependencies form a cycle. |
| FS016 | error | An integration ID does not match the ID format. |
| FS017 | error | The project name is blank. |
| FS018 | warning | The same entry appears twice in one list. |

## Normalized JSON

`framestack inspect` prints the manifest as normalized JSON: the same fields, with every list and table present even when empty. Other tools can read this form without knowing TOML or the defaults.

The structure is described by a JSON Schema (Draft 2020-12) at [`schemas/framestack-manifest-v1.schema.json`](https://github.com/cybercoretech/framestack/blob/main/schemas/framestack-manifest-v1.schema.json). The schema checks shape only; the rules in the table above are checked by `framestack check`.
