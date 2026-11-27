import type {ReactNode} from 'react';
import Link from '@docusaurus/Link';
import Layout from '@theme/Layout';

import styles from './index.module.css';

export default function Home(): ReactNode {
  return (
    <Layout
      title="Framework composition"
      description="FrameStack documentation and project architecture."
    >
      <main>
        <header className={`hero hero--primary ${styles.hero}`}>
          <div className="container">
            <p className={styles.eyebrow}>CYBERCORE TECH / DEVELOPER TOOLING</p>
            <h1 className={`hero__title ${styles.title}`}>FrameStack</h1>
            <p className={`hero__subtitle ${styles.subtitle}`}>
              Compose full-stack systems across frameworks and languages.
            </p>
            <div className={styles.actions}>
              <Link className="button button--primary button--lg" to="/docs/">
                Explore the documentation
              </Link>
              <Link
                className={`button button--outline button--secondary button--lg ${styles.secondaryAction}`}
                to="/docs/BLUEPRINT"
              >
                Read the architecture blueprint
              </Link>
            </div>
          </div>
        </header>

        <section className={`container ${styles.content}`}>
          <div className={styles.intro}>
            <p className={styles.kicker}>THE PROJECT MODEL</p>
            <h2>One project graph. Explicit composition.</h2>
            <p>
              FrameStack models applications, services, workers, and resources
              as components in a directed project graph. Capabilities,
              dependencies, communication relationships, and adapter
              operations are declared explicitly instead of being hidden
              inside a fixed frontend/backend pairing.
            </p>
          </div>

          <div className={styles.grid}>
            <article className={`card ${styles.card}`}>
              <div className="card__header">
                <p className={styles.cardIndex}>01 / GRAPH</p>
                <h3>Model the whole system</h3>
              </div>
              <div className="card__body">
                <p>
                  Describe component boundaries and the relationships that
                  determine build order, runtime requirements, and
                  communication.
                </p>
              </div>
            </article>

            <article className={`card ${styles.card}`}>
              <div className="card__header">
                <p className={styles.cardIndex}>02 / CAPABILITIES</p>
                <h3>Compose by contract</h3>
              </div>
              <div className="card__body">
                <p>
                  Match declared capabilities and constraints through
                  integration descriptors and structured operation plans.
                </p>
              </div>
            </article>

            <article className={`card ${styles.card}`}>
              <div className="card__header">
                <p className={styles.cardIndex}>03 / TOOLING</p>
                <h3>Keep execution explicit</h3>
              </div>
              <div className="card__body">
                <p>
                  Separate discovery, description, validation, planning,
                  execution, and reporting so workflows remain inspectable.
                </p>
              </div>
            </article>
          </div>

          <aside className={styles.status}>
            <span className={styles.statusMarker} aria-hidden="true" />
            <div>
              <strong>Architecture and foundation stage</strong>
              <p>
                Reference stacks and integration candidates describe the
                intended architecture. They are not claims of implemented
                adapter support. Support will be documented after
                implementation and verification.
              </p>
            </div>
          </aside>
        </section>
      </main>
    </Layout>
  );
}
