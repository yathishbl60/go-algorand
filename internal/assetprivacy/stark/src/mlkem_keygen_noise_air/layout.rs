// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub const ROWS: usize = 4096;
pub const PREPROCESSED: usize = 28;
pub(super) const RHO: usize = 256;
pub(super) const SIGMA: usize = 512;
pub(super) const COEFFICIENTS: usize = 768;
pub(super) const WORK: usize = COEFFICIENTS + 6 * 256;
pub(super) const SEED_WIDTH: usize = 3442;
pub(super) const NOISE_WIDTH: usize = 5522;
pub const WIDTH: usize = WORK + NOISE_WIDTH;

pub struct Wires<'a, T> {
    pub seed: &'a [T],
    pub rho: &'a [T],
    pub sigma: &'a [T],
    pub polynomials: &'a [T],
}

pub fn wires<T>(row: &[T]) -> Option<Wires<'_, T>> {
    if row.len() != WIDTH {
        return None;
    }
    Some(Wires {
        seed: &row[..RHO],
        rho: &row[RHO..SIGMA],
        sigma: &row[SIGMA..COEFFICIENTS],
        polynomials: &row[COEFFICIENTS..WORK],
    })
}
