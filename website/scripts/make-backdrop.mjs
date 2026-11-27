// Writes static/img/bg-editor.svg: a dim, static "code editor" backdrop
// (split panes, line numbers, syntax-coloured token bars, a minimap) for the
// FrameStack site. Deterministic, so re-running gives the same file.
// Usage: node scripts/make-backdrop.mjs
import { writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

let seed = 0x5f37;
const rand = () => ((seed = (seed * 1103515245 + 12345) & 0x7fffffff) / 0x7fffffff);
const pick = (xs) => xs[Math.floor(rand() * xs.length)];

const W = 1600, H = 1000, LINE = 22;
const colours = ['#2ee6f6', '#ff2e97', '#9b6bff', '#b6ff3b', '#ffb347', '#7d8a99', '#d7e0e9'];
const out = [];
out.push(`<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${W} ${H}" preserveAspectRatio="xMidYMin slice">`);
out.push('<defs><radialGradient id="g1" cx="18%" cy="12%" r="55%"><stop offset="0" stop-color="#2ee6f6" stop-opacity=".16"/><stop offset="1" stop-color="#2ee6f6" stop-opacity="0"/></radialGradient>');
out.push('<radialGradient id="g2" cx="86%" cy="78%" r="55%"><stop offset="0" stop-color="#ff2e97" stop-opacity=".13"/><stop offset="1" stop-color="#ff2e97" stop-opacity="0"/></radialGradient></defs>');
out.push(`<rect width="${W}" height="${H}" fill="#05070b"/>`);

// Panes: explorer | editor | editor | minimap
const panes = [[0, 210], [210, 860], [860, 1500], [1500, 1600]];
for (const [x0, x1] of panes.slice(1)) out.push(`<line x1="${x0}" y1="0" x2="${x0}" y2="${H}" stroke="#1a2431" stroke-width="1"/>`);
out.push(`<rect x="0" y="0" width="${W}" height="30" fill="#0a0f16"/><line x1="0" y1="30" x2="${W}" y2="30" stroke="#1a2431"/>`);
for (const [x0, label] of [[220, 'framestack.toml'], [870, 'plan.rs']]) {
  out.push(`<rect x="${x0}" y="4" width="132" height="26" fill="#0b1017" stroke="#1a2431"/>`);
  out.push(`<rect x="${x0 + 12}" y="14" width="${label.length * 6}" height="5" rx="2" fill="#7d8a99" opacity=".7"/>`);
}

// Explorer tree
for (let i = 0, y = 52; y < H - 20; i++, y += LINE) {
  const depth = pick([0, 1, 1, 2, 2, 3]);
  out.push(`<rect x="${16 + depth * 14}" y="${y}" width="${30 + Math.floor(rand() * 90)}" height="5" rx="2" fill="${depth === 0 ? '#2ee6f6' : '#556273'}" opacity="${depth === 0 ? 0.55 : 0.45}"/>`);
}

// Code panes with line numbers and token bars
for (const [x0, x1] of [panes[1], panes[2]]) {
  let indent = 0;
  for (let n = 1, y = 50; y < H - 10; n++, y += LINE) {
    out.push(`<rect x="${x0 + 14}" y="${y}" width="${String(n).length * 6}" height="5" rx="1" fill="#283647"/>`);
    if (rand() < 0.12) continue; // blank line
    indent = Math.max(0, Math.min(4, indent + pick([-1, 0, 0, 0, 1])));
    let x = x0 + 56 + indent * 22;
    const tokens = 1 + Math.floor(rand() * 6);
    for (let t = 0; t < tokens && x < x1 - 40; t++) {
      const w = 14 + Math.floor(rand() * 70);
      out.push(`<rect x="${x}" y="${y}" width="${Math.min(w, x1 - 30 - x)}" height="5" rx="2" fill="${pick(colours)}" opacity="${(0.32 + rand() * 0.3).toFixed(2)}"/>`);
      x += w + 8;
    }
  }
}

// Minimap
for (let y = 44; y < H - 10; y += 4) {
  out.push(`<rect x="1512" y="${y}" width="${10 + Math.floor(rand() * 70)}" height="2" fill="${pick(colours)}" opacity=".28"/>`);
}
out.push(`<rect x="1506" y="140" width="88" height="120" fill="#2ee6f6" opacity=".05" stroke="#2ee6f6" stroke-opacity=".18"/>`);
out.push(`<rect width="${W}" height="${H}" fill="url(#g1)"/><rect width="${W}" height="${H}" fill="url(#g2)"/>`);
out.push('</svg>\n');

const file = fileURLToPath(new URL('../static/img/bg-editor.svg', import.meta.url));
writeFileSync(file, out.join(''));
console.log(`wrote ${file}`);
