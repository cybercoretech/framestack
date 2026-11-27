/**
 * Content for the FrameStack homepage.
 *
 * Every terminal output below was captured from `framestack 0.1.0` (main at
 * e02a1f9) running on the manifests shown, on 2026-11-27. Re-capture them when
 * the CLI's output changes; do not edit them by hand.
 */

export type Edge = {from: string; to: string; type: 'build_dependency' | 'runtime_dependency' | 'communicates_with'; label?: string};
export type Node = {id: string; kind: string; detail: string};

export type Example = {
  id: string;
  label: string;
  file: string;
  status: 'builds' | 'valid' | 'invalid';
  summary: string;
  toml: string;
  nodes: Node[];
  edges: Edge[];
  outputs: {command: string; text: string; exit: number}[];
};

export const examples: Example[] = [
  {
    id: 'leptos-axum',
    label: 'rust-leptos-axum',
    file: 'examples/rust-leptos-axum/framestack.toml',
    status: 'builds',
    summary: 'The reference app. FrameStack plans, builds and serves it through the Leptos + Axum adapter; CI checks that it hydrates in Chrome.',
    toml: `# FrameStack manifest for the Leptos + Axum reference application.
manifest_version = 1

[project]
name = "rust-leptos-axum"
description = "Leptos server-side rendering with hydration, served by Axum"

[[components]]
id = "app"
kind = "application"
path = "."
language = "rust"
integrations = ["leptos", "axum"]
provides = ["frontend.ssr", "frontend.hydration", "http.server"]
env = ["LEPTOS_SITE_ADDR"]`,
    nodes: [{id: 'app', kind: 'application', detail: 'leptos + axum · rust'}],
    edges: [],
    outputs: [
      {
        command: 'framestack check --manifest examples/rust-leptos-axum',
        exit: 0,
        text: 'examples/rust-leptos-axum/framestack.toml: valid: 1 component, 0 relationships',
      },
      {
        command: 'framestack graph --manifest examples/rust-leptos-axum',
        exit: 0,
        text: `rust-leptos-axum: 1 component, 0 relationships

components
  app  application  .

relationships
  (none)

build order
  1. app`,
      },
      {
        command: 'framestack plan build --manifest examples/rust-leptos-axum',
        exit: 0,
        text: `plan: build (1 step)

app.build  build the server and the WebAssembly client with cargo-leptos
  $ cargo leptos build --release
  in examples/rust-leptos-axum/.
  writes target
  network crates.io

tools
  probe: cargo leptos --version  ->  cargo-leptos 0.3.12`,
      },
      {
        command: 'framestack plan dev --manifest examples/rust-leptos-axum',
        exit: 0,
        text: `plan: dev.start (1 step)

app.dev  serve the app and rebuild on changes with cargo-leptos
  $ cargo leptos watch
  in examples/rust-leptos-axum/.
  reads LEPTOS_SITE_ADDR
  writes target
  network crates.io
  listens on 127.0.0.1:3000
  runs until stopped

tools
  probe: cargo leptos --version  ->  cargo-leptos 0.3.12`,
      },
    ],
  },
  {
    id: 'mixed',
    label: 'mixed-reference',
    file: 'fixtures/manifest/valid/react-vite-axum.toml',
    status: 'valid',
    summary: 'A React + Vite front end, an Axum API and shared contracts. The graph and build order are real; planning stops at FS020 because those adapters are not built yet.',
    toml: `# ADR-0007 mixed-language proof: React + Vite frontend, Axum service.
manifest_version = 1

[project]
name = "mixed-reference"

[[components]]
id = "contracts"
kind = "application"
path = "contracts"
language = "rust"

[[components]]
id = "web"
kind = "application"
path = "web"
language = "typescript"
integrations = ["react", "vite"]
requires = ["api.rest"]

[[components]]
id = "api"
kind = "service"
path = "api"
language = "rust"
integrations = ["axum"]
provides = ["http.server", "api.rest"]

[[relationships]]
from = "web"
to = "api"
type = "communicates_with"
protocol = "http"

[[relationships]]
from = "web"
to = "contracts"
type = "build_dependency"

[[relationships]]
from = "api"
to = "contracts"
type = "build_dependency"`,
    nodes: [
      {id: 'contracts', kind: 'application', detail: 'rust'},
      {id: 'web', kind: 'application', detail: 'react + vite · typescript'},
      {id: 'api', kind: 'service', detail: 'axum · rust'},
    ],
    edges: [
      {from: 'web', to: 'api', type: 'communicates_with', label: 'http'},
      {from: 'web', to: 'contracts', type: 'build_dependency'},
      {from: 'api', to: 'contracts', type: 'build_dependency'},
    ],
    outputs: [
      {
        command: 'framestack check --manifest fixtures/manifest/valid/react-vite-axum.toml',
        exit: 0,
        text: 'fixtures/manifest/valid/react-vite-axum.toml: valid: 3 components, 3 relationships',
      },
      {
        command: 'framestack graph --manifest fixtures/manifest/valid/react-vite-axum.toml',
        exit: 0,
        text: `mixed-reference: 3 components, 3 relationships

components
  contracts  application  contracts
  web  application  web
  api  service  api

relationships
  web -> api  communicates_with (http)
  web -> contracts  build_dependency
  api -> contracts  build_dependency

build order
  1. contracts
  2. web
  3. api`,
      },
      {
        command: 'framestack plan build --manifest fixtures/manifest/valid/react-vite-axum.toml',
        exit: 1,
        text: `error[FS020] components[1].integrations: no adapter handles the integrations [react, vite] of component \`web\`
error[FS020] components[2].integrations: no adapter handles the integrations [axum] of component \`api\`
2 errors, 0 warnings`,
      },
    ],
  },
  {
    id: 'cycle',
    label: 'build-cycle',
    file: 'fixtures/manifest/invalid/build-cycle.toml',
    status: 'invalid',
    summary: 'Two services that each need the other built first. check names the cycle before anything runs.',
    toml: `manifest_version = 1

[project]
name = "fixture"

[[components]]
id = "a"
kind = "service"
path = "a"

[[components]]
id = "b"
kind = "service"
path = "b"

[[relationships]]
from = "a"
to = "b"
type = "build_dependency"

[[relationships]]
from = "b"
to = "a"
type = "build_dependency"`,
    nodes: [
      {id: 'a', kind: 'service', detail: 'no integrations'},
      {id: 'b', kind: 'service', detail: 'no integrations'},
    ],
    edges: [
      {from: 'a', to: 'b', type: 'build_dependency'},
      {from: 'b', to: 'a', type: 'build_dependency'},
    ],
    outputs: [
      {
        command: 'framestack check --manifest fixtures/manifest/invalid/build-cycle.toml',
        exit: 1,
        text: `error[FS007] relationships: build dependencies form a cycle: a -> b -> a
fixtures/manifest/invalid/build-cycle.toml: 1 error, 0 warnings`,
      },
    ],
  },
];

export type Feature = {
  id: string;
  index: string;
  title: string;
  blurb: string;
  file: string;
  status: string;
  points: string[];
  links: {label: string; to: string}[];
};

export const features: Feature[] = [
  {
    id: 'manifest',
    index: '01',
    title: 'One manifest for the whole system',
    blurb: 'framestack.toml lists every component, what it provides and needs, and how the pieces relate.',
    file: 'framestack.toml',
    status: 'shipped · core + CLI',
    points: [
      'Components carry kind, path or external, language, integrations, provides, requires and env.',
      'Relationships are build_dependency, runtime_dependency or communicates_with, with an optional protocol.',
      'framestack init writes a starter manifest; inspect prints the normalized JSON form.',
    ],
    links: [{label: 'Manifest reference', to: '/docs/reference/manifest'}],
  },
  {
    id: 'check',
    index: '02',
    title: '24 checks before anything runs',
    blurb: 'check reports every problem at once, each with a stable FS code, a location and plain words.',
    file: 'diagnostics.rs',
    status: 'shipped · FS001–FS024',
    points: [
      'Parse errors point at the line and column; graph errors name the cycle (a -> b -> a).',
      'Required capabilities must be provided by the component or something it depends on.',
      '--format json gives editors and scripts the same diagnostics.',
    ],
    links: [{label: 'Validation rules', to: '/docs/reference/manifest#validation-rules'}],
  },
  {
    id: 'graph',
    index: '03',
    title: 'A real project graph',
    blurb: 'graph prints the components, the relationships and the order things must be built in.',
    file: 'graph.rs',
    status: 'shipped · text, json, mermaid',
    points: [
      'Build order puts every component after everything it has a build_dependency on.',
      'Mermaid output renders straight into GitHub and these docs.',
      'Runtime cycles are a warning (FS015); build cycles are an error (FS007).',
    ],
    links: [{label: 'CLI: graph', to: '/docs/reference/cli#framestack-graph'}],
  },
  {
    id: 'plan',
    index: '04',
    title: 'Plan first. Run only with --yes.',
    blurb: 'plan shows every command, its directory, the variables it reads, what it writes, which hosts it contacts and what it listens on.',
    file: 'plan.rs',
    status: 'shipped · plan / build / dev',
    points: [
      'The only programs plan starts are the tools’ version probes, such as cargo leptos --version.',
      'build and dev print the plan and exit 2 without --yes. With it, steps run as programs, never through a shell.',
      'A missing tool is FS021 with the install command; a failed step is FS022 with a per-step summary.',
    ],
    links: [{label: 'CLI: plan, build, dev', to: '/docs/reference/cli#framestack-plan'}],
  },
  {
    id: 'adapters',
    index: '05',
    title: 'Adapters own the how',
    blurb: 'An adapter knows how to build and run one kind of component. The core picks it, checks capabilities and collects its steps.',
    file: 'adapter.rs',
    status: 'shipped · leptos-axum',
    points: [
      'A component goes to the adapter whose integration set matches exactly.',
      'Capabilities, operations and settings are checked before planning (FS019, FS023, FS024).',
      'Adapters are compiled in. Loading them from elsewhere waits for a trust model.',
    ],
    links: [{label: 'Adapters', to: '/docs/reference/adapters'}],
  },
  {
    id: 'proof',
    index: '06',
    title: 'Proved in CI, end to end',
    blurb: 'Every change builds the Leptos + Axum reference app through FrameStack, serves it and checks it hydrates.',
    file: 'smoke-leptos-axum.sh',
    status: 'shipped · CI job',
    points: [
      'The smoke test checks /healthz, the server-rendered page and the WebAssembly bundle.',
      'Headless Chrome loads the page and waits for "Hydrated in the browser", text only the client can set.',
      'The example pins wasm-bindgen =0.2.129 to match the CLI cargo-leptos uses.',
    ],
    links: [{label: 'Reference app', to: 'https://github.com/cybercoretech/framestack/tree/main/examples/rust-leptos-axum'}],
  },
];

export type AdapterEntry = {name: string; status: 'shipped' | 'planned'; note: string};
export type AdapterGroup = {group: string; entries: AdapterEntry[]};

/** Shipped adapters, then the reference stacks ADR-0007 names as next. */
export const adapters: AdapterGroup[] = [
  {
    group: 'rust',
    entries: [
      {name: 'leptos-axum', status: 'shipped', note: 'SSR + hydration · build, dev.start · cargo-leptos 0.3.12'},
      {name: 'axum', status: 'planned', note: 'HTTP service on its own (reference stack, ADR-0007)'},
    ],
  },
  {
    group: 'typescript',
    entries: [
      {name: 'react-vite', status: 'planned', note: 'Browser SPA (reference stack, ADR-0007)'},
      {name: 'fastify', status: 'planned', note: 'HTTP API (reference stack, ADR-0007)'},
    ],
  },
];

export type Diagnostic = {code: string; level: 'error' | 'warning'; text: string; stage: 'manifest' | 'plan' | 'run'};

export const diagnostics: Diagnostic[] = [
  {code: 'FS001', level: 'error', stage: 'manifest', text: 'Not valid TOML, a required field is missing, a value has the wrong type, or a field is unknown.'},
  {code: 'FS002', level: 'error', stage: 'manifest', text: 'manifest_version is not 1.'},
  {code: 'FS003', level: 'error', stage: 'manifest', text: 'A component ID does not match the ID format.'},
  {code: 'FS004', level: 'error', stage: 'manifest', text: 'Two components have the same ID.'},
  {code: 'FS005', level: 'error', stage: 'manifest', text: 'A relationship names a component that does not exist.'},
  {code: 'FS006', level: 'error', stage: 'manifest', text: 'A relationship goes from a component to itself.'},
  {code: 'FS007', level: 'error', stage: 'manifest', text: 'Build dependencies form a cycle, such as a -> b -> a.'},
  {code: 'FS008', level: 'error', stage: 'manifest', text: 'A capability ID is not in the vocabulary.'},
  {code: 'FS009', level: 'error', stage: 'manifest', text: 'A required capability is not provided by the component or anything it depends on.'},
  {code: 'FS010', level: 'error', stage: 'manifest', text: 'A component has neither path nor external, or has both.'},
  {code: 'FS011', level: 'error', stage: 'manifest', text: 'A path is absolute, contains .., or uses backslashes.'},
  {code: 'FS012', level: 'error', stage: 'manifest', text: 'A config table is for an integration the component does not list.'},
  {code: 'FS013', level: 'error', stage: 'manifest', text: 'An env entry is not a valid variable name.'},
  {code: 'FS014', level: 'warning', stage: 'manifest', text: 'The same relationship is declared more than once.'},
  {code: 'FS015', level: 'warning', stage: 'manifest', text: 'Runtime dependencies form a cycle.'},
  {code: 'FS016', level: 'error', stage: 'manifest', text: 'An integration ID does not match the ID format.'},
  {code: 'FS017', level: 'error', stage: 'manifest', text: 'The project name is blank.'},
  {code: 'FS018', level: 'warning', stage: 'manifest', text: 'The same entry appears twice in one list.'},
  {code: 'FS019', level: 'error', stage: 'plan', text: 'A component declares a capability its adapter does not provide.'},
  {code: 'FS020', level: 'error', stage: 'plan', text: 'No adapter handles the component’s integrations.'},
  {code: 'FS021', level: 'error', stage: 'plan', text: 'A required tool is missing, or its version command fails.'},
  {code: 'FS022', level: 'error', stage: 'run', text: 'A step failed, was stopped by a signal, or could not start.'},
  {code: 'FS023', level: 'error', stage: 'plan', text: 'An integration setting is not supported by its adapter.'},
  {code: 'FS024', level: 'error', stage: 'plan', text: 'The adapter cannot perform the requested operation for this component.'},
];

/** The typed hero session: real commands, captured output (trimmed to key lines). */
export const heroSession: {kind: 'cmd' | 'ok' | 'warn' | 'out'; text: string; dim?: string}[] = [
  {kind: 'cmd', text: 'framestack check --manifest examples/rust-leptos-axum'},
  {kind: 'ok', text: 'valid', dim: '1 component, 0 relationships'},
  {kind: 'cmd', text: 'framestack plan build --manifest examples/rust-leptos-axum'},
  {kind: 'out', text: 'app.build', dim: '$ cargo leptos build --release'},
  {kind: 'ok', text: 'probe', dim: 'cargo leptos --version  ->  cargo-leptos 0.3.12'},
  {kind: 'cmd', text: 'framestack build --manifest examples/rust-leptos-axum'},
  {kind: 'warn', text: 'nothing ran', dim: 'plan shown, exit 2: add --yes'},
  {kind: 'cmd', text: 'framestack build --yes --manifest examples/rust-leptos-axum'},
];
