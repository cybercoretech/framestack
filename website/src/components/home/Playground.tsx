import {useState, type ReactNode} from 'react';
import {examples} from '@site/src/data/site';
import GraphView from './GraphView';
import {highlightToml} from './highlight';
import styles from './home.module.css';

const STATUS: Record<string, string> = {builds: 'builds + runs', valid: 'valid · no adapter yet', invalid: 'invalid'};

export default function Playground(): ReactNode {
  const [exampleId, setExampleId] = useState(examples[0].id);
  const [outputIndex, setOutputIndex] = useState(0);
  const example = examples.find((e) => e.id === exampleId) ?? examples[0];
  const output = example.outputs[Math.min(outputIndex, example.outputs.length - 1)];

  return (
    <div className={styles.playground}>
      <div className={styles.exampleTabs} role="tablist" aria-label="Example manifests">
        {examples.map((e) => (
          <button
            key={e.id}
            type="button"
            role="tab"
            aria-selected={e.id === example.id}
            className={styles.exampleTab}
            data-status={e.status}
            onClick={() => {
              setExampleId(e.id);
              setOutputIndex(0);
            }}
          >
            <span className={styles.tabName}>{e.label}</span>
            <span className={styles.tabStatus}>{STATUS[e.status]}</span>
          </button>
        ))}
      </div>
      <p className={styles.exampleSummary}>{example.summary}</p>

      <div className={styles.panes}>
        <div className={styles.editor}>
          <div className={styles.windowBar}>
            <i /><i /><i />
            <span>{example.file}</span>
          </div>
          <pre className={styles.code} tabIndex={0} aria-label={`${example.file} contents`}>
            {example.toml.split('\n').map((line, i) => (
              <span key={i} className={styles.codeLine}>
                <span className={styles.ln} aria-hidden="true">{i + 1}</span>
                <span>{highlightToml(line)}</span>
              </span>
            ))}
          </pre>
        </div>

        <div className={styles.side}>
          <GraphView nodes={example.nodes} edges={example.edges} title={`Project graph of ${example.label}`} />
          <div className={styles.output}>
            <div className={styles.outputTabs} role="tablist" aria-label="Commands">
              {example.outputs.map((o, i) => (
                <button key={o.command} type="button" role="tab" aria-selected={o === output} className={styles.outputTab} onClick={() => setOutputIndex(i)}>
                  {o.command.split(' ').slice(1, o.command.startsWith('framestack plan') ? 3 : 2).join(' ')}
                </button>
              ))}
            </div>
            <pre className={styles.outputBody} tabIndex={0} aria-live="polite">
              <span className={styles.prompt}>❯ </span>
              <span className={styles.cmd}>{output.command}</span>
              {'\n'}
              {output.text.split('\n').map((line, i) => (
                <span key={i} className={line.startsWith('error') ? styles.errLine : line.startsWith('  $') ? styles.cmdLine : undefined}>
                  {line}
                  {'\n'}
                </span>
              ))}
              <span className={output.exit === 0 ? styles.exitOk : styles.exitBad}>exit {output.exit}</span>
            </pre>
          </div>
        </div>
      </div>
      <p className={styles.captured}>// output captured from framestack 0.1.0 on these exact manifests; nothing here is mocked up</p>
    </div>
  );
}
