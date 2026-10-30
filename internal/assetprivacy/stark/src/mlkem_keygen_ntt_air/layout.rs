// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{mlkem_keygen_noise_air as noise, mlkem_ntt_air as ntt};

pub const ROWS: usize = 16384;
pub(crate) const FRAME_ROWS: usize = 10240;
pub(super) const NOISE_ROWS: usize = 4096;
pub(super) const NOISE_WIDTH: usize = 7826;
pub(super) const RAW: usize = 768;
pub(super) const CORE: usize = RAW + 6 * 256;
pub(super) const WORK: usize = CORE;
pub(super) const NTT_ROWS: usize = 1024;
pub(super) const NTT_WIDTH: usize = 920;
pub(super) const NTT_SCHEDULE: usize = 514;
pub(super) const NOISE_SCHEDULE: usize = 28;
pub(super) const FLAGS: usize = NOISE_SCHEDULE + NTT_SCHEDULE;
pub(super) const TRANSFORMED: usize = NOISE_WIDTH;
pub const PREPROCESSED: usize = FLAGS + 7 * 4;
pub const WIDTH: usize = TRANSFORMED + 6 * 256;
const _: () = assert!(
    noise::WIDTH == NOISE_WIDTH
        && noise::ROWS == NOISE_ROWS
        && noise::PREPROCESSED == NOISE_SCHEDULE
);
const _: () = assert!(ntt::WIDTH == NTT_WIDTH && ntt::SCHEDULE_WIDTH == NTT_SCHEDULE);
const _: () = assert!(NOISE_ROWS + 6 * NTT_ROWS <= ROWS);

pub struct Wires<'a, T> {
    pub seed: &'a [T],
    pub rho: &'a [T],
    pub sigma: &'a [T],
    pub polynomials: &'a [T],
    pub transformed: &'a [T],
}

pub fn wires<T>(row: &[T]) -> Option<Wires<'_, T>> {
    if row.len() != WIDTH {
        return None;
    }
    Some(Wires {
        seed: &row[..256],
        rho: &row[256..512],
        sigma: &row[512..RAW],
        polynomials: &row[RAW..CORE],
        transformed: &row[TRANSFORMED..WIDTH],
    })
}
