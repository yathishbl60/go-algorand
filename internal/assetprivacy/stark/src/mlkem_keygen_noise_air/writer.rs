// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::globals::write_globals;
use super::{MlKemKeygenNoiseAir, ROWS, WIDTH, WORK};
use crate::{Value, mlkem_keygen_seed_air::writer::expand};
use p3_field::PrimeCharacteristicRing;

impl MlKemKeygenNoiseAir {
    pub(crate) fn write_into(
        &self,
        seed: [u8; 32],
        values: &mut [Value],
        target: (usize, usize, usize),
    ) -> Option<()> {
        check_target(values.len(), target)?;
        let (_, expanded) = expand(seed);
        let sigma = expanded[32..].try_into().ok()?;
        let coefficients = core::array::from_fn(|i| self.noise[i].values(sigma));
        clear_frame(values, target);
        self.write_components(seed, sigma, values, target)?;
        write_globals(values, target, seed, expanded, coefficients);
        Some(())
    }
    fn write_components(
        &self,
        seed: [u8; 32],
        sigma: [u8; 32],
        values: &mut [Value],
        target: (usize, usize, usize),
    ) -> Option<()> {
        let (stride, column, first) = target;
        self.seed
            .write_into(seed, values, (stride, column + WORK, first))?;
        for i in 0..6 {
            self.noise[i].write_into(
                sigma,
                values,
                (stride, column + WORK, first + (i + 1) * 512),
            )?;
        }
        Some(())
    }
}

fn check_target(len: usize, target: (usize, usize, usize)) -> Option<()> {
    let (stride, column, first) = target;
    if stride == 0 || !len.is_multiple_of(stride) || column.checked_add(WIDTH)? > stride {
        return None;
    }
    (first.checked_add(ROWS)? <= len / stride).then_some(())
}

fn clear_frame(values: &mut [Value], target: (usize, usize, usize)) {
    for row in values.chunks_exact_mut(target.0).skip(target.2).take(ROWS) {
        row[target.1..target.1 + WIDTH].fill(Value::ZERO);
    }
}
