import {useState, type ReactNode} from 'react';
import {adapters} from '@site/src/data/site';
import styles from './home.module.css';

/** Explorer-style tree of adapters: what ships today and what is planned next. */
export default function AdapterTree(): ReactNode {
  const all = adapters.flatMap((g) => g.entries.map((e) => ({...e, group: g.group})));
  const [selected, setSelected] = useState(all[0].name);
  const current = all.find((a) => a.name === selected) ?? all[0];
  return (
    <div className={styles.explorer}>
      <div className={styles.tree} role="tree" aria-label="Adapters">
        <p className={styles.treeHead}>EXPLORER · integrations/</p>
        {adapters.map((g) => (
          <div key={g.group} role="group" aria-label={g.group}>
            <p className={styles.treeGroup}>▾ {g.group}/</p>
            {g.entries.map((e) => (
              <button
                key={e.name}
                type="button"
                role="treeitem"
                aria-selected={e.name === selected}
                className={styles.treeItem}
                data-status={e.status}
                onClick={() => setSelected(e.name)}
              >
                <span className={styles.treeDot} aria-hidden="true" />
                {e.name}
                <small>{e.status}</small>
              </button>
            ))}
          </div>
        ))}
      </div>
      <div className={styles.treePreview} aria-live="polite">
        <div className={styles.windowBar}>
          <i /><i /><i />
          <span>{current.group}/{current.name}</span>
        </div>
        <div className={styles.treePreviewBody}>
          <p className={styles.kicker} data-status={current.status}>
            // {current.status === 'shipped' ? 'shipped · in CI on every change' : 'planned · not built yet'}
          </p>
          <h3>{current.name}</h3>
          <p>{current.note}</p>
          {current.status === 'shipped' ? (
            <pre className={styles.miniCode}>{`[[components]]
id = "app"
integrations = ["leptos", "axum"]
provides = ["frontend.ssr", "frontend.hydration", "http.server"]`}</pre>
          ) : (
            <p className={styles.honest}>
              <b>// note</b> a manifest can already name these integrations, and check and graph handle them. plan reports FS020 until the adapter exists.
            </p>
          )}
        </div>
      </div>
    </div>
  );
}
