# FrameStack Documentation Standards

**Status:** Project standard  
**Applies to:** Public documentation, internal technical documentation where relevant, API references, examples, and release documentation  
**Canonical source:** Repository `docs/` directory  
**Publishing application:** Repository `website/` directory, powered by Docusaurus 3.x

## 1. Purpose and principles

FrameStack documentation is part of the product. It must help users understand what the project does, install and configure it, compose supported integrations, diagnose problems, and understand the behavior and constraints of the system.

Documentation must follow these principles:

1. **Correctness over appearance.** A polished page is not useful if it describes behavior that does not exist.
2. **One canonical source.** Content is maintained in `docs/`; the website renders it.
3. **Specificity.** Explain FrameStack's actual contracts, commands, configuration, and behavior. Avoid generic advice where project-specific guidance is required.
4. **Completeness.** Explain prerequisites, steps, expected outcomes, failure modes, and verification.
5. **Traceability.** Version-sensitive statements must identify the relevant release, implementation status, or compatibility boundary.
6. **Maintainability.** Prefer reusable terminology and stable links. Avoid duplicating the same reference material across pages.
7. **Progressive disclosure.** Introduce concepts before relying on them in advanced guides, while keeping reference material precise.
8. **Honest status.** Clearly distinguish implemented, tested, experimental, planned, deprecated, and unsupported functionality.

Documentation must never imply that a planned feature, adapter, compatibility combination, or command is implemented merely because it appears in the architecture or roadmap.

## 2. Content ownership and repository layout

### 2.1 Canonical content

All maintained documentation content belongs under `docs/`. The `website/` directory owns the Docusaurus application, configuration, theme, custom React components, static assets, and publishing workflow. It must not become a second manually maintained copy of the documentation.

The documentation tree may contain public product documentation and internal engineering material. Docusaurus configuration must explicitly include the intended public content and exclude internal-only material, including agent handoff instructions and local workflow notes.

### 2.2 Recommended information architecture

Create directories and pages when there is substantive content to publish. The following is the target information architecture, not a requirement to create empty directories or placeholder pages immediately.

| Area | Purpose |
| --- | --- |
| `getting-started/` | Installation, prerequisites, first project, first successful workflow |
| `concepts/` | Terminology, component model, project graph, capabilities, adapters, lifecycle |
| `guides/` | Task-oriented workflows and practical recipes |
| `integrations/` | Framework, runtime, database, messaging, and adapter-specific guidance |
| `reference/` | CLI, manifest, configuration, schemas, public types, errors, and compatibility |
| `architecture/` | System design, internal boundaries, data flow, invariants, and rationale |
| `security/` | Threat model, secrets, execution safety, permissions, and reporting |
| `contributing/` | Development setup, testing, code review, documentation contribution |
| `releases/` | Release notes, deprecations, migration guides, and support policy |
| `decisions/` | Accepted and superseded architecture decision records where approved for publication |

Existing project-planning documents and agent-only material must not be exposed as public pages by accident. Their publication status should be explicit.

### 2.3 Page metadata

Use Docusaurus-compatible frontmatter consistently. At minimum, pages should have a clear title. Add a description when it improves search results, previews, or page context. Use stable IDs only when a stable identifier is needed for links or navigation.

Do not add meaningless metadata solely to satisfy a template. If CI later requires particular metadata, document the rule and validate it consistently.

## 3. Writing and editorial standards

### 3.1 Voice and terminology

- Use direct, technically precise language.
- Prefer active voice and explicit subjects.
- Define project-specific terms before relying on them.
- Use the same term for the same concept throughout the documentation.
- Preserve distinctions in the architecture. For example, a component kind is not an integration role; a capability is not an adapter operation; and build, runtime, and communication relationships are not interchangeable.
- Expand an acronym on first use unless it is universally understood by the intended audience.
- Avoid marketing claims that cannot be substantiated by implementation and tests.

### 3.2 Page structure

Choose a structure appropriate to the page's purpose. Do not force every page into an identical template. Where relevant, include:

1. Purpose and scope
2. Prerequisites and supported versions
3. Conceptual explanation
4. Procedure or configuration
5. Complete examples
6. Expected result and verification
7. Limitations and compatibility notes
8. Troubleshooting and common errors
9. Related pages

Use headings in a logical hierarchy. Do not skip heading levels for visual effect. Keep headings descriptive and useful in a table of contents.

### 3.3 Accuracy and status language

Use explicit status labels or prose where readers could otherwise misunderstand maturity:

- **Implemented:** present in the codebase.
- **Tested:** covered by identified automated or documented verification.
- **Experimental:** available but subject to change and not covered by normal stability expectations.
- **Planned:** accepted or proposed work that is not yet implemented.
- **Deprecated:** still present but scheduled for removal or replacement.
- **Unsupported:** intentionally outside the project's supported behavior.

Do not use “supported” as a synonym for “theoretically possible.” State the tested version range and combinations when known. If compatibility has not been tested, say so.

### 3.4 Links and cross-references

- Use descriptive link text rather than “click here.”
- Prefer links to canonical pages instead of copying their content.
- Check relative paths and anchors after moving or renaming pages.
- Avoid linking to unstable external resources when an official, durable source exists.
- External links should make sense to readers and should not be required to understand essential project behavior.
- Do not leave TODO links, placeholder URLs, or links to pages that do not exist in published navigation.

## 4. Tutorials and task-oriented guides

A tutorial should lead a reader to a verifiable result. A guide should help a reader complete a defined task.

Where applicable, every tutorial or guide must state:

- Intended audience and goal
- Prerequisites, including required tool versions
- Starting repository or project assumptions
- Exact commands and configuration changes
- Relevant working directory for commands
- Expected output or observable result
- How to verify success
- Cleanup or reversal steps when resources are created
- Common failure modes and recovery steps
- Links to deeper conceptual and reference material

Commands must be complete enough to run in the stated context. Do not omit required configuration or silently assume environment variables, files, services, or credentials.

If output is shortened, mark it as abbreviated and preserve the important lines. Never invent terminal output and present it as a verified result.

## 5. Code and configuration examples

### 5.1 General requirements

- Examples must reflect real FrameStack syntax and current implementation.
- Prefer complete, minimal examples over disconnected fragments.
- Identify the language and relevant file path.
- Explain non-obvious choices and required dependencies.
- Do not include real credentials, tokens, private keys, personal data, or production secrets.
- Use clearly fake values for illustrative credentials.
- Distinguish pseudocode from executable code.
- Identify version requirements and feature status where relevant.

### 5.2 Verification

Where practical, examples should be compiled, executed, or validated in CI. The documentation must state when an example is illustrative and not automatically tested.

Prefer maintainable examples that can be referenced from documentation and exercised by tests. Avoid copying large code blocks into multiple pages when a canonical example file can be linked or included.

### 5.3 Shell examples

- State the shell and operating-system assumptions when they matter.
- Use safe quoting and explicit working-directory context.
- Explain commands with destructive or irreversible effects before presenting them.
- Never make destructive commands appear routine or harmless.
- Avoid commands that silently overwrite user data.
- Do not expose machine-specific paths, personal usernames, or local secrets in examples.

### 5.4 TOML and manifest examples

Manifest documentation must agree with the authoritative parser, schema, and semantic validation rules. For every documented field, specify when known:

- Field name and location
- Data type
- Required or optional status
- Default value
- Allowed values or format
- Constraints and cross-field rules
- Version introduced or deprecated
- A valid example
- Relevant validation errors

Do not document schema validation as a substitute for semantic validation. Explain graph integrity, cycle detection, capability satisfaction, composition, and compatibility rules separately where applicable.

## 6. CLI reference requirements

Each documented CLI command or subcommand should include:

- Command name and purpose
- Full syntax
- Positional arguments
- Options and flags
- Defaults and environment-variable behavior
- Required project or working-directory context
- Examples
- Expected output or artifacts
- Exit codes
- Error conditions and recovery guidance
- Stability or experimental status

The reference must match the actual command implementation. Do not publish speculative flags or options. When command behavior changes, update examples, reference pages, and migration guidance in the same change.

## 7. API and generated reference documentation

### 7.1 General policy

Generated documentation supplements maintained explanations; it does not replace them. A useful API reference explains not only symbol names but also intent, lifecycle, constraints, error behavior, and compatibility.

### 7.2 Rust

Use rustdoc for Rust crates and public items where applicable. Public types, traits, functions, and modules should have meaningful documentation comments. Include examples for important public APIs where practical. Document invariants, ownership/lifetime expectations, error behavior, and feature-gating where relevant.

### 7.3 TypeScript

Use a TypeDoc-compatible pipeline where applicable. Public types, functions, classes, and package entry points should have meaningful documentation comments and examples. Document asynchronous behavior, thrown/rejected errors, optional properties, and compatibility expectations.

### 7.4 Schemas and generated references

Where possible, derive manifest and protocol reference material from authoritative schemas or definitions. Generated output must be deterministic, reviewed, and linked to the source version from which it was produced.

### 7.5 Custom FrameStack contracts

Document the purpose, inputs, outputs, invariants, lifecycle, compatibility policy, and failure behavior for FrameStack-owned contracts. In particular, keep the distinctions clear between:

- Project graph component kinds and integration roles
- Capabilities and adapter operations
- Descriptor declarations (`provides`, `requires`, and structured constraints)
- Build dependencies, runtime dependencies, and communication relationships
- Planning and execution
- Validation errors, execution failures, and reported diagnostics

Do not promise a stable public API before the project has deliberately established and tested its compatibility policy.

## 8. Integration and adapter documentation

Every integration page must describe the actual integration and its tested state. Where applicable, include:

1. Integration name and purpose
2. Implementation and support status
3. Supported framework/runtime versions
4. Prerequisites and installation
5. Component kind and integration role
6. Capabilities provided and required
7. Descriptor and configuration format
8. Constraints and compatibility rules
9. Supported adapter operations
10. Development, build, test, and execution workflows
11. A complete working example
12. Diagnostics and troubleshooting
13. Known limitations
14. Related integrations and references

Do not infer support from a framework's own capabilities. FrameStack support requires a FrameStack integration implementation and project verification. Clearly distinguish an official reference stack from an implemented adapter or a tested compatibility claim.

## 9. Architecture and decision documentation

Architecture pages should explain:

- The problem and scope
- Responsibilities and boundaries
- Relevant interfaces and data structures
- Data flow and lifecycle
- Invariants and constraints
- Failure modes and recovery behavior
- Security implications
- Alternatives and trade-offs
- Links to relevant implementation and tests

Architecture Decision Records (ADRs) must use stable numbering and include status, date, context, decision, consequences, and reconsideration criteria. Superseded decisions should remain available and link to the decision that replaced them. Do not silently rewrite historical rationale to match later implementation.

## 10. Security documentation

Security documentation must be explicit about trust boundaries and execution behavior. Cover, as applicable:

- Secret references and environment handling
- Credential storage and exposure risks
- Adapter discovery and trust
- Validation versus execution
- Plan review and approval
- Process invocation and command execution
- Filesystem and network access
- Logs, diagnostics, and redaction
- Dependency and supply-chain risks
- Reporting vulnerabilities

Never include real secrets or sensitive user data in examples, screenshots, logs, or test fixtures. Avoid implying that a validation step provides a security guarantee unless the implementation and threat model support that claim.

## 11. Accessibility and website experience

The Docusaurus website and custom theme must remain usable with keyboard navigation, screen readers, zoom, and narrow viewports.

- Use semantic HTML and accessible names for controls.
- Preserve visible keyboard focus.
- Maintain sufficient text and interface contrast.
- Provide useful alternative text for informative images; use empty alternative text for purely decorative images.
- Do not communicate status through color alone.
- Ensure code blocks, tables, diagrams, and custom React components remain readable on mobile.
- Avoid custom components when standard Markdown communicates the information clearly.
- Interactive examples must have an understandable non-interactive explanation or fallback.

The custom FrameStack/CyberCore theme must not compromise readability or accessibility for visual branding.

## 12. Search, navigation, and discoverability

Navigation should reflect reader tasks and the information architecture. Keep labels concise and predictable. Avoid deep nesting where a flatter structure is clearer.

Integrated search must index the intended public documentation and provide useful page titles and descriptions. Internal-only material must not appear in public search results. Search behavior and indexing must be checked as part of website validation.

## 13. Versioning, releases, and migration guides

### 13.1 Documentation snapshots

Publish a documentation snapshot alongside each FrameStack release. The snapshot must correspond to the released software, not the latest development branch. Keep previous supported snapshots discoverable.

### 13.2 Release notes

Release notes should identify:

- Added, changed, fixed, deprecated, and removed behavior
- Compatibility and dependency changes
- Security-relevant changes
- Known issues and limitations
- Required user action
- Links to affected documentation and migration guides

### 13.3 Migration guides

A migration guide is required when a release introduces a breaking change or a change that requires meaningful user action. It should explain:

1. Which versions and users are affected
2. What changed and why
3. How the old behavior or configuration differs
4. Exact migration steps
5. Before-and-after examples where useful
6. Validation steps to confirm a successful migration
7. Rollback or mitigation guidance when available
8. Known caveats and links to related changes

Migration guides must be retained and remain accessible after newer releases. Do not replace old migration guidance with a link to the latest guide.

### 13.4 Deprecations

Document the affected item, reason, replacement, deprecation version, expected removal version if known, and migration path. Do not state a removal date unless it has been decided.

## 14. Documentation contribution workflow

A documentation change should be reviewed with the same care as code.

Before submitting a change, contributors should:

- Confirm that the documented behavior exists or clearly label it as planned.
- Update all affected pages and cross-references.
- Check commands, examples, paths, versions, and configuration against implementation.
- Run the available Markdown/MDX lint and link checks.
- Build the Docusaurus site when the website is configured.
- Verify generated references when affected.
- Check rendering, navigation, and accessibility for custom components.
- Include release or migration documentation when the change requires it.

Reviewers should check technical accuracy, completeness, terminology, links, and whether the page answers the reader's likely next question.

## 15. CI validation requirements

Documentation CI should be introduced incrementally and must test what it claims to test.

The target checks are:

| Check | Expected validation |
| --- | --- |
| Markdown lint | Consistent Markdown style and valid structure |
| MDX build | MDX parses and the site compiles |
| Link validation | Internal links, anchors, and configured external links |
| Frontmatter | Required metadata is present and valid |
| Navigation | Sidebar and navigation references resolve |
| Production build | Docusaurus can generate the production site |
| Code examples | Selected examples compile, execute, or validate |
| Generated API docs | Reference generation completes and output is current |
| Manifest reference | Published fields agree with authoritative schema/definitions |
| Release snapshots | Versioned docs correspond to release artifacts |
| Migration coverage | Breaking changes include migration guidance |

External-link checks may need retry or allowlist handling for transient network failures. Such handling must not hide broken internal links.

CI must not silently rewrite canonical documentation. Formatting fixes should be made in reviewed source changes.

## 16. Definition of done for documentation changes

A documentation change is complete when all applicable conditions are met:

- The content accurately reflects implementation or clearly identifies planned behavior.
- The page has a clear purpose and appropriate structure.
- Terminology matches the rest of FrameStack documentation.
- Examples and commands are complete and verified where practical.
- Links and anchors resolve.
- Relevant navigation and metadata are updated.
- Sensitive information is absent.
- Accessibility has been considered, especially for custom components.
- Applicable lint, link, build, and example checks pass.
- Release notes or migration guidance are included when required.

A page should not be marked complete merely because it exists or appears in the sidebar.
