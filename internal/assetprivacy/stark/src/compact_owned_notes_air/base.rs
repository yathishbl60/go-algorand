// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{OwnedNotesAir, schema::*};
use p3_air::BaseAir;
use p3_field::PrimeField64;
use std::borrow::Cow;

impl<F: PrimeField64> BaseAir<F> for OwnedNotesAir<F> {
    fn width(&self) -> usize {
        WIDTH
    }
    fn num_public_values(&self) -> usize {
        64
    }
    fn num_periodic_columns(&self) -> usize {
        PERIODIC
    }
    fn periodic_columns(&self) -> Cow<'_, [Vec<F>]> {
        Cow::Borrowed(&self.periodic)
    }
    fn main_next_row_columns(&self) -> Vec<usize> {
        [
            vec![NOTE + 8, NOTE + 9],
            (HASH..HASH + 1920).collect(),
            (OWNER..OWNER + 32).collect(),
        ]
        .concat()
    }
    fn max_constraint_degree(&self) -> Option<usize> {
        Some(5)
    }
}
