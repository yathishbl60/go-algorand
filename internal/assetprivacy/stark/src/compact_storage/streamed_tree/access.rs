// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{StreamedTree, TreeCap, TreeDigest, Value32};

impl StreamedTree {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn widths(&self) -> &[usize] {
        &self.widths
    }

    pub fn cap(&self) -> Option<TreeCap> {
        Some(TreeCap::new(vec![*self.layers.last()?.first()?]))
    }

    /// Siblings in the pinned binary MMCS's bottom-up order, indexed by bit-reversed row.
    pub fn path(&self, mut row: usize) -> Option<Vec<TreeDigest>> {
        if row >= self.rows {
            return None;
        }
        let mut path = Vec::new();
        path.try_reserve_exact(self.layers.len() - 1).ok()?;
        for layer in &self.layers[..self.layers.len() - 1] {
            path.push(*layer.get(row ^ 1)?);
            row >>= 1;
        }
        Some(path)
    }

    /// Reveals only the requested query's independent per-matrix salt rows.
    pub fn salts(&self, row: usize) -> Option<Vec<Vec<Value32>>> {
        if row >= self.rows {
            return None;
        }
        self.salts
            .iter()
            .map(|matrix| matrix.get(row).map(|salt| salt.to_vec()))
            .collect()
    }
}
