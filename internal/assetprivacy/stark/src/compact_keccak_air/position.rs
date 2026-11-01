// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CompactKeccakAir, ROUNDS};
use p3_field::PrimeField64;

/// Identity comes only from a constructor-owned fixed row schedule.
#[derive(Clone, Copy)]
pub struct RowPosition {
    permutation: usize,
    round: usize,
}

impl RowPosition {
    pub fn permutation_index(self) -> usize {
        self.permutation
    }
    pub fn round_index(self) -> usize {
        self.round
    }
    pub fn is_first(self) -> bool {
        self.round == 0
    }
    pub fn is_last(self) -> bool {
        self.round == ROUNDS - 1
    }
}

impl<F: PrimeField64> CompactKeccakAir<F> {
    pub fn position(&self, row: usize) -> Option<RowPosition> {
        (row < self.count * ROUNDS).then_some(RowPosition {
            permutation: row / ROUNDS,
            round: row % ROUNDS,
        })
    }
}
