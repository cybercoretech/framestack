# FrameStack Agent Handoff

> Copy this file to the repository root as `AGENT_HANDOFF.md`.
> The root file is intentionally gitignored and must never be committed.

## Project identity

- Project: FrameStack
- Repository: `cybercoretech/framestack`
- Local workspace: `/home/raven/Devspace/Cybercore/tooling/framestack`
- Current stage: Stage 0 — Blueprint and decisions

## Read first

1. `PROJECT_STATE.md`
2. `docs/BLUEPRINT.md`
3. `docs/ROADMAP.md`
4. `docs/decisions/`

## Confirmed direction

- Multi-language full-stack framework and development ecosystem.
- Rust-native frontend candidates include Leptos and Dioxus.
- TypeScript frontend candidates include React and other established options.
- Multiple backend runtimes should integrate through explicit adapters.
- Keep core independent of framework-specific dependencies.
- Prefer versioned contracts and shared cross-language fixtures.
- Do not claim candidate integrations are implemented or supported.

## Working rules

- Inspect the repository and current Git status before editing.
- Preserve user changes; never overwrite or reset uncommitted work.
- Use complete, testable vertical slices.
- Record durable architectural decisions in ADRs.
- Keep this handoff file local; put durable status in `PROJECT_STATE.md`.
- Never store credentials, tokens, or secret values here.

## Current task

Update this section at the start of each agent session.

## Changes made

Record files changed, commands run, and test results.

## Remaining work

Record the next concrete actions and unresolved questions.
