import {useEffect, useRef, useState, type ReactNode} from 'react';
import Link from '@docusaurus/Link';
import {features, type Feature} from '@site/src/data/site';
import styles from './home.module.css';

/** Feature cards; each opens a <dialog> sheet with the detail behind it. */
export default function FeatureSheets(): ReactNode {
  const dialog = useRef<HTMLDialogElement>(null);
  const opener = useRef<HTMLButtonElement | null>(null);
  const [active, setActive] = useState<Feature | null>(null);

  useEffect(() => {
    if (active && dialog.current && !dialog.current.open) dialog.current.showModal();
  }, [active]);

  const close = () => {
    dialog.current?.close();
  };

  return (
    <>
      <div className={styles.cards}>
        {features.map((f) => (
          <button
            key={f.id}
            type="button"
            className={styles.card}
            data-reveal
            onClick={(e) => {
              opener.current = e.currentTarget;
              setActive(f);
            }}
            aria-haspopup="dialog"
          >
            <span className={styles.cardTop}>
              <span className={styles.cardIndex}>{f.index}</span>
              <span className={styles.cardFile}>{f.file}</span>
            </span>
            <span className={styles.cardTitle}>{f.title}</span>
            <span className={styles.cardBlurb}>{f.blurb}</span>
            <span className={styles.cardStatus}>● {f.status}</span>
            <span className={styles.cardOpen} aria-hidden="true">open ↗</span>
          </button>
        ))}
      </div>

      <dialog
        ref={dialog}
        className={styles.sheet}
        aria-labelledby="fs-sheet-title"
        data-lenis-prevent
        onClose={() => {
          setActive(null);
          opener.current?.focus();
        }}
        onClick={(e) => {
          if (e.target === dialog.current) close();
        }}
      >
        {active && (
          <div className={styles.sheetInner}>
            <div className={styles.windowBar}>
              <i /><i /><i />
              <span>{active.file}</span>
              <button type="button" className={styles.sheetClose} onClick={close} aria-label="Close">✕</button>
            </div>
            <div className={styles.sheetBody}>
              <p className={styles.kicker}>// {active.index} · {active.status}</p>
              <h3 id="fs-sheet-title">{active.title}</h3>
              <p className={styles.sheetLede}>{active.blurb}</p>
              <ul className={styles.sheetPoints}>
                {active.points.map((p) => <li key={p}>{p}</li>)}
              </ul>
              <div className={styles.sheetLinks}>
                {active.links.map((l) => (
                  <Link key={l.to} className={styles.btnGhost} to={l.to}>{l.label} →</Link>
                ))}
              </div>
            </div>
          </div>
        )}
      </dialog>
    </>
  );
}
