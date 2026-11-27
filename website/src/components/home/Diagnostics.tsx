import {useState, type ReactNode} from 'react';
import {diagnostics} from '@site/src/data/site';
import styles from './home.module.css';

const STAGES = [
  {id: 'all', label: 'all 24'},
  {id: 'manifest', label: 'check'},
  {id: 'plan', label: 'plan'},
  {id: 'run', label: 'run'},
] as const;

/** The FS001–FS024 board with a stage filter and a detail line. */
export default function Diagnostics(): ReactNode {
  const [stage, setStage] = useState<(typeof STAGES)[number]['id']>('all');
  const [code, setCode] = useState('FS007');
  const shown = diagnostics.filter((d) => stage === 'all' || d.stage === stage);
  const current = diagnostics.find((d) => d.code === code) ?? diagnostics[0];
  return (
    <div className={styles.diag}>
      <div className={styles.diagFilters} role="group" aria-label="Filter by stage">
        {STAGES.map((s) => (
          <button key={s.id} type="button" aria-pressed={stage === s.id} className={styles.chip} onClick={() => setStage(s.id)}>
            {s.label}
          </button>
        ))}
      </div>
      <div className={styles.diagGrid}>
        {shown.map((d) => (
          <button
            key={d.code}
            type="button"
            className={styles.diagCell}
            data-level={d.level}
            aria-pressed={d.code === code}
            onClick={() => setCode(d.code)}
            onMouseEnter={() => setCode(d.code)}
            onFocus={() => setCode(d.code)}
          >
            {d.code}
          </button>
        ))}
      </div>
      <p className={styles.diagDetail} aria-live="polite">
        <b data-level={current.level}>{current.level}[{current.code}]</b> <span className={styles.dim}>{current.stage}</span> {current.text}
      </p>
    </div>
  );
}
