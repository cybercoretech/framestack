# Contributing to FrameStack

FrameStack is in its blueprint phase. Contributions should strengthen the architecture before expanding the implementation surface.

## Before changing code

1. Read `PROJECT_STATE.md`.
2. Read `docs/BLUEPRINT.md` and the relevant decision records.
3. Check open questions and existing decisions before introducing a new dependency or abstraction.
4. Keep changes focused and explain the architectural reason for non-obvious choices.

## Architecture expectations

- Keep the core independent of optional frontend and backend frameworks.
- Integrate external frameworks through adapters with explicit capabilities.
- Prefer stable, versioned contracts at language boundaries.
- Use shared fixtures to verify equivalent behavior across implementations.
- Avoid speculative plugin systems and premature abstraction.
- Make security-sensitive behavior explicit and default-deny where practical.

## Pull requests

- Include tests for behavior changes.
- Update documentation and decision records when architecture changes.
- Keep public APIs small and document compatibility implications.
- Ensure CI passes before requesting review.
- Never include secrets, generated build output, or the local `AGENT_HANDOFF.md`.

## Local agent handoff

`AGENT_HANDOFF.md` is intentionally ignored by Git. It may contain temporary task context for local coding agents. Use `docs/agent/AGENT_HANDOFF.template.md` as the starting point. Durable project decisions belong in tracked documentation.
