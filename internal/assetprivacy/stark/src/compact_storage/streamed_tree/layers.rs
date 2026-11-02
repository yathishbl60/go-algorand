// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::TreeDigest;
use crate::compact::compact_compress;

pub(super) fn build(leaves: Vec<TreeDigest>) -> Option<Vec<Vec<TreeDigest>>> {
    if !leaves.len().is_power_of_two() {
        return None;
    }
    let mut layers = Vec::new();
    layers
        .try_reserve_exact(leaves.len().ilog2() as usize + 1)
        .ok()?;
    layers.push(leaves);
    while layers.last()?.len() > 1 {
        layers.push(parents(layers.last()?)?);
    }
    Some(layers)
}

fn parents(children: &[TreeDigest]) -> Option<Vec<TreeDigest>> {
    let mut parents = Vec::new();
    parents.try_reserve_exact(children.len() / 2).ok()?;
    for pair in children.as_chunks::<2>().0 {
        parents.push(compact_compress(pair[0], pair[1]));
    }
    Some(parents)
}
