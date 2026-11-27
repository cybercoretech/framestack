import type {ReactNode} from 'react';
import type {Edge, Node} from '@site/src/data/site';
import styles from './home.module.css';

const MIN_NODE_W = 150;
const CHAR_W = 6.6; // width of one 10.5px JetBrains Mono character
const NODE_H = 58;
const GAP_X = 90;
const GAP_Y = 26;

/**
 * Layers nodes by build order: a component sits one column right of everything
 * it has a build_dependency on. Nodes in a cycle fall back to manifest order.
 */
function layout(nodes: Node[], edges: Edge[], NODE_W: number) {
  const deps = new Map(nodes.map((n) => [n.id, edges.filter((e) => e.type === 'build_dependency' && e.from === n.id).map((e) => e.to)]));
  const layer = new Map<string, number>();
  const visiting = new Set<string>();
  let cyclic = false;
  const depth = (id: string): number => {
    if (layer.has(id)) return layer.get(id)!;
    if (visiting.has(id)) {
      cyclic = true;
      return 0;
    }
    visiting.add(id);
    const d = Math.max(-1, ...(deps.get(id) ?? []).map(depth)) + 1;
    visiting.delete(id);
    layer.set(id, d);
    return d;
  };
  nodes.forEach((n) => depth(n.id));
  if (cyclic) nodes.forEach((n, i) => layer.set(n.id, i));
  const columns = new Map<number, string[]>();
  nodes.forEach((n) => columns.set(layer.get(n.id)!, [...(columns.get(layer.get(n.id)!) ?? []), n.id]));
  const tallest = Math.max(...Array.from(columns.values(), (c) => c.length));
  const pos = new Map<string, {x: number; y: number}>();
  columns.forEach((ids, col) => {
    const offset = ((tallest - ids.length) * (NODE_H + GAP_Y)) / 2;
    ids.forEach((id, row) => pos.set(id, {x: 20 + col * (NODE_W + GAP_X), y: 20 + offset + row * (NODE_H + GAP_Y)}));
  });
  const width = 40 + columns.size * NODE_W + (columns.size - 1) * GAP_X;
  const height = 40 + tallest * NODE_H + (tallest - 1) * GAP_Y;
  return {pos, width, height, cyclic};
}

export default function GraphView({nodes, edges, title}: {nodes: Node[]; edges: Edge[]; title: string}): ReactNode {
  // Wide enough for the longest "kind · detail" line, so text never spills out of a box.
  const NODE_W = Math.max(MIN_NODE_W, ...nodes.map((n) => Math.ceil((n.kind.length + 3 + n.detail.length) * CHAR_W) + 24));
  const {pos, width, height, cyclic} = layout(nodes, edges, NODE_W);
  const viewW = Math.max(width, 260);
  return (
    <figure className={styles.graph}>
      {/* Drawn at its natural size (never scaled up), shrinking only when the pane is narrower. */}
      <svg viewBox={`0 0 ${viewW} ${height + 10}`} width={viewW} height={height + 10} role="img" aria-label={title}>
        <defs>
          {(['build', 'talk', 'bad'] as const).map((k) => (
            <marker key={k} id={`fs-arrow-${k}`} viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
              <path d="M0,0 L10,5 L0,10 z" className={styles[`arrow_${k}`]} />
            </marker>
          ))}
        </defs>
        {edges.map((e, i) => {
          const a = pos.get(e.from)!;
          const b = pos.get(e.to)!;
          const back = b.x <= a.x;
          const kind = cyclic && e.type === 'build_dependency' ? 'bad' : e.type === 'communicates_with' ? 'talk' : 'build';
          // Dependencies point at what is built first, so draw from the dependent to its dependency.
          const x1 = back ? a.x + NODE_W / 2 : a.x;
          const y1 = back ? a.y + (i % 2 ? NODE_H : 0) : a.y + NODE_H / 2;
          const x2 = back ? b.x + NODE_W / 2 : b.x + NODE_W;
          const y2 = back ? b.y + (i % 2 ? NODE_H : 0) : b.y + NODE_H / 2;
          const bend = back ? (i % 2 ? 46 : -46) : 0;
          const d = back
            ? `M${x1},${y1} C${x1},${y1 + bend} ${x2},${y2 + bend} ${x2},${y2}`
            : `M${x1},${y1} C${(x1 + x2) / 2},${y1} ${(x1 + x2) / 2},${y2} ${x2},${y2}`;
          return (
            <g key={i}>
              <path d={d} className={styles[`edge_${kind}`]} markerEnd={`url(#fs-arrow-${kind})`} />
              {e.label && (
                <text x={(x1 + x2) / 2} y={(y1 + y2) / 2 - 6} className={styles.edgeLabel} textAnchor="middle">{e.label}</text>
              )}
            </g>
          );
        })}
        {nodes.map((n) => {
          const p = pos.get(n.id)!;
          return (
            <g key={n.id} transform={`translate(${p.x},${p.y})`}>
              <rect width={NODE_W} height={NODE_H} className={n.kind === 'service' ? styles.nodeService : styles.nodeApp} />
              <text x="12" y="23" className={styles.nodeId}>{n.id}</text>
              <text x="12" y="42" className={styles.nodeDetail}>{n.kind} · {n.detail}</text>
            </g>
          );
        })}
      </svg>
      <figcaption className={styles.legend}>
        <span><i className={styles.lgBuild} />build_dependency</span>
        <span><i className={styles.lgTalk} />communicates_with</span>
        {cyclic && <span><i className={styles.lgBad} />cycle (FS007)</span>}
      </figcaption>
    </figure>
  );
}
