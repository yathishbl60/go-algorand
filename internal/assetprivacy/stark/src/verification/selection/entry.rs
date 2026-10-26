// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::Layout;
use crate::Config;
use p3_uni_stark::PreprocessedVerifierKey;

/// A small public preprocessing-key record, produced by setup or a pinned loader.
#[derive(Clone)]
pub struct Entry {
    pub(super) layout: Layout,
    pub(super) key: PreprocessedVerifierKey<Config>,
}

impl Entry {
    pub(super) fn new(layout: Layout, key: PreprocessedVerifierKey<Config>) -> Result<Self, Error> {
        if key.width != layout.shape().preprocessed
            || key.degree_bits != layout.rows().ilog2() as usize + 1
            || key.commitment.num_roots() != 1
        {
            return Err(Error::Artifact);
        }
        Ok(Self { layout, key })
    }

    pub fn layout(&self) -> Layout {
        self.layout
    }
}
