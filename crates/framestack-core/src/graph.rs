//! Graph queries over a manifest's components and relationships.

use crate::manifest::{Manifest, RelationshipType};

/// Outgoing edges of one relationship type, by component index. Edges whose
/// endpoints do not exist are skipped; validation reports those separately.
fn adjacency(manifest: &Manifest, kind: RelationshipType) -> Vec<Vec<usize>> {
    let index = |id: &str| manifest.components.iter().position(|c| c.id == id);
    let mut edges = vec![Vec::new(); manifest.components.len()];
    for relationship in manifest.relationships.iter().filter(|r| r.kind == kind) {
        if let (Some(from), Some(to)) = (index(&relationship.from), index(&relationship.to)) {
            if !edges[from].contains(&to) {
                edges[from].push(to);
            }
        }
    }
    edges
}

/// One cycle among relationships of `kind`, as component IDs with the first
/// repeated at the end, such as `["a", "b", "a"]`. Components are visited in
/// manifest order, so the same manifest always reports the same cycle.
pub fn find_cycle(manifest: &Manifest, kind: RelationshipType) -> Option<Vec<String>> {
    #[derive(Clone, Copy, PartialEq)]
    enum Visit {
        New,
        OnPath,
        Done,
    }

    let edges = adjacency(manifest, kind);
    let mut visit = vec![Visit::New; edges.len()];

    for start in 0..edges.len() {
        if visit[start] != Visit::New {
            continue;
        }
        // Depth-first search without recursion: each entry is a component and
        // the index of the next outgoing edge to follow.
        let mut path: Vec<(usize, usize)> = vec![(start, 0)];
        visit[start] = Visit::OnPath;
        while let Some(top) = path.last_mut() {
            let node = top.0;
            let Some(&target) = edges[node].get(top.1) else {
                visit[node] = Visit::Done;
                path.pop();
                continue;
            };
            top.1 += 1;
            match visit[target] {
                Visit::New => {
                    visit[target] = Visit::OnPath;
                    path.push((target, 0));
                }
                Visit::OnPath => {
                    let begin = path.iter().position(|&(n, _)| n == target).unwrap_or(0);
                    let mut cycle: Vec<String> = path[begin..]
                        .iter()
                        .map(|&(n, _)| manifest.components[n].id.clone())
                        .collect();
                    cycle.push(manifest.components[target].id.clone());
                    return Some(cycle);
                }
                Visit::Done => {}
            }
        }
    }
    None
}

/// Component IDs in an order where each component comes after everything it
/// has a `build_dependency` on. Among components that are ready at the same
/// time, manifest order wins. `None` when the build graph has a cycle.
pub fn build_order(manifest: &Manifest) -> Option<Vec<String>> {
    let edges = adjacency(manifest, RelationshipType::BuildDependency);
    let count = edges.len();
    // waiting_on[n] = build dependencies of n that are not placed yet.
    let mut waiting_on: Vec<usize> = edges.iter().map(Vec::len).collect();
    let mut placed = vec![false; count];
    let mut order = Vec::with_capacity(count);

    while order.len() < count {
        let ready = (0..count).find(|&n| !placed[n] && waiting_on[n] == 0)?;
        placed[ready] = true;
        order.push(manifest.components[ready].id.clone());
        for (node, targets) in edges.iter().enumerate() {
            if targets.contains(&ready) {
                waiting_on[node] -= 1;
            }
        }
    }
    Some(order)
}
