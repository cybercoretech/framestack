import type {ReactNode} from 'react';
import styles from './home.module.css';

/** Minimal TOML colouring for the manifest panes: comments, tables, keys, strings, numbers. */
export function highlightToml(line: string): ReactNode {
  if (/^\s*#/.test(line)) return <span className={styles.tComment}>{line}</span>;
  const table = line.match(/^(\s*)(\[\[?[^\]]+\]\]?)(.*)$/);
  if (table) return <>{table[1]}<span className={styles.tTable}>{table[2]}</span>{table[3]}</>;
  const kv = line.match(/^(\s*)([A-Za-z_][\w.-]*)(\s*=\s*)(.*)$/);
  if (!kv) return line || ' ';
  const value = kv[4].split(/("(?:[^"\\]|\\.)*")/g).map((part, i) =>
    part.startsWith('"') ? (
      <span key={i} className={styles.tString}>{part}</span>
    ) : (
      <span key={i} className={/^\s*\d+\s*$/.test(part) ? styles.tNumber : styles.tPunct}>{part}</span>
    ),
  );
  return <>{kv[1]}<span className={styles.tKey}>{kv[2]}</span><span className={styles.tPunct}>{kv[3]}</span>{value}</>;
}
