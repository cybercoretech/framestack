import {useState, type ReactNode} from 'react';
import Link from '@docusaurus/Link';
import Layout from '@theme/Layout';
import Terminal from '@site/src/components/home/Terminal';
import Playground from '@site/src/components/home/Playground';
import FeatureSheets from '@site/src/components/home/FeatureSheets';
import AdapterTree from '@site/src/components/home/AdapterTree';
import Diagnostics from '@site/src/components/home/Diagnostics';
import {useReveal, useScramble} from '@site/src/components/home/motion';
import styles from '@site/src/components/home/home.module.css';

const QUICKSTART = [
  {label: 'install the CLI (Rust 1.85+)', cmd: 'cargo install --path crates/framestack-cli'},
  {label: 'check the reference app', cmd: 'framestack check --manifest examples/rust-leptos-axum'},
  {label: 'see exactly what a build would run', cmd: 'framestack plan build --manifest examples/rust-leptos-axum'},
  {label: 'build it (needs cargo-leptos 0.3.12)', cmd: 'framestack build --yes --manifest examples/rust-leptos-axum'},
  {label: 'serve on 127.0.0.1:3000, rebuild on change', cmd: 'framestack dev --yes --manifest examples/rust-leptos-axum'},
];

function SectionHead({index, kicker, title, id, children}: {index: string; kicker: string; title: string; id: string; children?: ReactNode}): ReactNode {
  const ref = useScramble<HTMLHeadingElement>();
  return (
    <header className={styles.sectionHead}>
      <p className={styles.kicker}>// {index} · {kicker}</p>
      <h2 id={id} ref={ref}>
        <span data-text={title}>{title}</span>
      </h2>
      {children && <p className={styles.sectionLede}>{children}</p>}
    </header>
  );
}

function CopyLine({label, cmd}: {label: string; cmd: string}): ReactNode {
  const [copied, setCopied] = useState(false);
  return (
    <li className={styles.qsLine}>
      <span className={styles.qsLabel}># {label}</span>
      <span className={styles.qsCmd}>
        <code>
          <b className={styles.prompt}>$ </b>
          {cmd}
        </code>
        <button
          type="button"
          className={styles.copy}
          aria-label={`Copy: ${cmd}`}
          onClick={async () => {
            try {
              await navigator.clipboard.writeText(cmd);
              setCopied(true);
              setTimeout(() => setCopied(false), 1400);
            } catch {
              setCopied(false);
            }
          }}
        >
          {copied ? 'copied' : 'copy'}
        </button>
      </span>
    </li>
  );
}

export default function Home(): ReactNode {
  useReveal();
  const heroRef = useScramble<HTMLHeadingElement>();
  return (
    <Layout
      title="Compose the stack. See the plan."
      description="FrameStack describes a whole system in one manifest, checks it with 24 diagnostics, prints a plan of every command, and runs it only with --yes. The Leptos + Axum adapter builds and serves a real app today."
    >
      <main className={styles.home}>
        <section className={styles.hero} id="top">
          <div className={styles.heroCopy}>
            <p className={styles.eyebrow}>
              <span className={styles.pulse} aria-hidden="true" />
              cybercore / framestack · v0.1.0 · rust
            </p>
            <h1 className={styles.heroTitle} ref={heroRef}>
              <span data-text="Compose the stack.">Compose the stack.</span>{' '}
              <em><span data-text="See the plan.">See the plan.</span></em>
            </h1>
            <p className={styles.lede}>
              One manifest describes every part of your system: what each component provides, what it needs and how the
              parts connect. FrameStack checks it, draws the graph, prints every command a build would run, and runs nothing
              until you say <code>--yes</code>.
            </p>
            <div className={styles.cta}>
              <a className={styles.btnPrimary} href="#quickstart">$ get started</a>
              <a className={styles.btnGhost} href="#playground">▶ try the manifests</a>
              <Link className={styles.btnGhost} to="/docs/">read the docs</Link>
            </div>
            <dl className={styles.stats}>
              <div><dt>commands</dt><dd>7</dd></div>
              <div><dt>checks</dt><dd>24</dd></div>
              <div><dt>adapters in CI</dt><dd>1</dd></div>
              <div><dt>runs w/o --yes</dt><dd>0</dd></div>
            </dl>
          </div>
          <Terminal />
        </section>

        <section className={styles.section} aria-labelledby="works-h">
          <SectionHead index="01" kicker="what works today" title="Shipped, tested, in CI." id="works-h">
            Core library, CLI and the first adapter are merged. Open any card for the detail behind it.
          </SectionHead>
          <FeatureSheets />
        </section>

        <section className={styles.section} id="playground" aria-labelledby="play-h">
          <SectionHead index="02" kicker="manifest → graph → plan" title="Read the manifest. Watch the plan." id="play-h">
            Three real manifests from the repository: one that builds and runs, one that is valid but waits on adapters, and
            one that check rejects.
          </SectionHead>
          <Playground />
        </section>

        <section className={styles.section} aria-labelledby="adapters-h">
          <SectionHead index="03" kicker="adapters" title="The core plans. Adapters know how." id="adapters-h">
            A component goes to the adapter whose integrations match exactly. One adapter ships today; the reference stacks
            from ADR-0007 come next.
          </SectionHead>
          <AdapterTree />
        </section>

        <section className={styles.section} aria-labelledby="diag-h">
          <SectionHead index="04" kicker="diagnostics" title="24 ways to fail early." id="diag-h">
            Every problem gets a stable code, a location and plain words, whether check finds it, plan finds it, or a step
            fails at run time.
          </SectionHead>
          <Diagnostics />
        </section>

        <section className={styles.section} id="quickstart" aria-labelledby="qs-h">
          <SectionHead index="05" kicker="quickstart" title="From clone to a hydrated app." id="qs-h" />
          <div className={styles.qs} data-reveal>
            <div className={styles.windowBar}>
              <i /><i /><i />
              <span>zsh — ~/framestack</span>
            </div>
            <ol className={styles.qsList}>
              {QUICKSTART.map((q) => <CopyLine key={q.cmd} {...q} />)}
            </ol>
          </div>
          <p className={styles.honest}>
            <b>// note</b> FrameStack is early: one adapter (Leptos + Axum), no published releases yet, tested on Linux.
            Install from a clone of <a href="https://github.com/cybercoretech/framestack" target="_blank" rel="noopener">cybercoretech/framestack<span className="sr-only"> (opens in a new tab)</span></a>.
          </p>
        </section>
      </main>
    </Layout>
  );
}
