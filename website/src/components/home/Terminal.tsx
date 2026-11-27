import {useEffect, useState, type ReactNode} from 'react';
import {heroSession} from '@site/src/data/site';
import {useReducedMotion} from './motion';
import styles from './home.module.css';

/** The hero's typed terminal session. Every line is visible without JavaScript. */
export default function Terminal(): ReactNode {
  const reduce = useReducedMotion();
  const [shown, setShown] = useState(heroSession.length);
  const [typed, setTyped] = useState<number | null>(null);

  useEffect(() => {
    if (reduce) {
      setShown(heroSession.length);
      setTyped(null);
      return;
    }
    let cancelled = false;
    let timer: ReturnType<typeof setTimeout>;
    setShown(0);
    const step = (line: number, chars: number) => {
      if (cancelled || line >= heroSession.length) return;
      const entry = heroSession[line];
      if (entry.kind === 'cmd' && chars < entry.text.length) {
        setShown(line);
        setTyped(chars);
        timer = setTimeout(() => step(line, chars + 2), 18);
        return;
      }
      setTyped(null);
      setShown(line + 1);
      timer = setTimeout(() => step(line + 1, 0), entry.kind === 'cmd' ? 380 : 240);
    };
    timer = setTimeout(() => step(0, 0), 500);
    return () => {
      cancelled = true;
      clearTimeout(timer);
    };
  }, [reduce]);

  return (
    <div className={styles.term} aria-label="Example FrameStack session">
      <div className={styles.windowBar}>
        <i /><i /><i />
        <span>zsh — ~/framestack</span>
      </div>
      <pre className={styles.termBody}>
        {heroSession.map((line, i) => {
          if (i > shown || (i === shown && typed === null)) return null;
          const text = i === shown && typed !== null ? line.text.slice(0, typed) : line.text;
          return (
            <span key={i} className={styles.termLine}>
              {line.kind === 'cmd' && <b className={styles.prompt}>❯ </b>}
              {line.kind === 'ok' && <b className={styles.ok}>✓ </b>}
              {line.kind === 'warn' && <b className={styles.warn}>! </b>}
              {line.kind === 'out' && <b className={styles.arrow}>→ </b>}
              <span className={line.kind === 'cmd' ? styles.cmd : undefined}>{text}</span>
              {line.dim && <span className={styles.dim}>  {line.dim}</span>}
            </span>
          );
        })}
        <span className={styles.termLine}>
          <b className={styles.prompt}>❯ </b>
          <span className={styles.caret} aria-hidden="true" />
        </span>
      </pre>
    </div>
  );
}
