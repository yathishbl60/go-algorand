// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Error, bundle::MAX_SLOTS};
use super::Layout;
use crate::{NUM_QUERIES, RANDOM_CODEWORDS};

impl Layout {
    /// Necessary query-row wire bytes, not a complete proof-size bound.
    /// Pinned hiding PCS adds random columns; duplicate queries retain row values.
    /// Canonical field values use eight-byte u64 encoding, regardless of bitness.
    pub fn minimum_query_bytes(self) -> Result<usize, Error> {
        let shape = self.shape();
        let columns = shape
            .width
            .checked_add(shape.preprocessed)
            .and_then(|n| n.checked_add(RANDOM_CODEWORDS));
        columns
            .and_then(|n| n.checked_mul(NUM_QUERIES))
            .and_then(|n| n.checked_mul(8))
            .ok_or(Error::ProofBudget)
    }
}

/// Sum a proposed native bundle's necessary query bytes without constructing AIRs.
/// This checks count/overflow, not operation ordering or native authorization.
/// Omits quotient/random openings, OOD, paths, salts, framing and transaction data.
pub fn minimum_bundle_query_bytes(layouts: &[Layout]) -> Result<usize, Error> {
    if layouts.is_empty() || layouts.len() > MAX_SLOTS {
        return Err(Error::Shape);
    }
    layouts.iter().try_fold(0usize, |sum, layout| {
        sum.checked_add(layout.minimum_query_bytes()?)
            .ok_or(Error::ProofBudget)
    })
}
