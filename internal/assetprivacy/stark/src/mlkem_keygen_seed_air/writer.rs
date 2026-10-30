// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DIGEST, MlKemKeygenSeedAir, RHO, ROWS, WIDTH};
use crate::{Value, sha3_air::digest_values};
use p3_field::PrimeCharacteristicRing;
use sha3::{Digest, Sha3_512};

pub(crate) fn expand(seed: [u8; 32]) -> ([u8; 33], [u8; 64]) {
    let mut message = [0; 33];
    message[..32].copy_from_slice(&seed);
    message[32] = 3;
    (message, Sha3_512::digest(message).into())
}

impl MlKemKeygenSeedAir {
    pub(crate) fn write_into(
        &self,
        seed: [u8; 32],
        values: &mut [Value],
        target: (usize, usize, usize),
    ) -> Option<()> {
        let (stride, column, first) = target;
        check_target(values.len(), target)?;
        let (message, digest) = expand(seed);
        self.hash.write_frame(&message, values, target, ROWS)?;
        for parent in values.chunks_exact_mut(stride).skip(first).take(ROWS) {
            write_output(&mut parent[column..column + WIDTH], digest);
        }
        Some(())
    }
}

fn write_output(row: &mut [Value], digest: [u8; 64]) {
    row[DIGEST..RHO].copy_from_slice(&digest_values(digest));
    for (bit, value) in row[RHO..WIDTH].iter_mut().enumerate() {
        *value = Value::from_bool(digest[bit / 8] & (1 << (bit % 8)) != 0);
    }
}

fn check_target(len: usize, target: (usize, usize, usize)) -> Option<()> {
    let (stride, column, first) = target;
    if stride == 0 || !len.is_multiple_of(stride) || column.checked_add(WIDTH)? > stride {
        return None;
    }
    (first.checked_add(ROWS)? <= len / stride).then_some(())
}
