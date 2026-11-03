// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{StreamedTree, TreeDigest};
use p3_merkle_tree::PrunedMerklePaths;
use std::collections::BTreeSet;

impl StreamedTree {
    /// Minimal boundary digests in stock binary frontier order; query indices stay external.
    pub fn pruned_path(&self, indices: &[usize]) -> Option<PrunedMerklePaths<u64, 8>> {
        if indices.iter().any(|index| *index >= self.rows) {
            return None;
        }
        let mut frontier: BTreeSet<_> = indices.iter().copied().collect();
        let mut siblings = Vec::new();
        for layer in &self.layers[..self.layers.len() - 1] {
            append_boundary(layer, &frontier, &mut siblings)?;
            frontier = frontier.iter().map(|index| index >> 1).collect();
        }
        Some(PrunedMerklePaths {
            sibling_hashes: siblings,
        })
    }
}

fn append_boundary(
    layer: &[TreeDigest],
    frontier: &BTreeSet<usize>,
    siblings: &mut Vec<TreeDigest>,
) -> Option<()> {
    let missing = frontier
        .iter()
        .filter(|index| !frontier.contains(&(**index ^ 1)))
        .count();
    siblings.try_reserve(missing).ok()?;
    for index in frontier {
        if !frontier.contains(&(index ^ 1)) {
            siblings.push(*layer.get(index ^ 1)?);
        }
    }
    Some(())
}
