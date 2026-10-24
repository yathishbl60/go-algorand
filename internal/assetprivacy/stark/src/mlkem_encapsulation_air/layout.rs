// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{COINS, KEY, RHO, WIDTH, WORK};

/// Secret wires for later key authorization and envelope composition.
pub struct Wires<'a, T> {
    pub public_polynomials: &'a [T],
    pub rho_bits: &'a [T],
    pub message_bits: &'a [T],
    pub shared_secret_bits: &'a [T],
    pub coins_bits: &'a [T],
}

pub fn wires<T>(row: &[T]) -> Option<Wires<'_, T>> {
    (row.len() == WIDTH).then(|| Wires {
        public_polynomials: &row[2304..3072],
        rho_bits: &row[RHO..KEY],
        message_bits: &row[7953..8209],
        shared_secret_bits: &row[KEY..COINS],
        coins_bits: &row[COINS..WORK],
    })
}
