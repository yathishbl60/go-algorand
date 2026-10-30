// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::globals::write_globals;
use super::{CORE, FRAME_ROWS, MlKemKeygenNttAir, NOISE_ROWS, NTT_ROWS, RAW, ROWS, WIDTH, WORK};
use crate::Value;
use p3_field::{PrimeCharacteristicRing, PrimeField64};

impl MlKemKeygenNttAir {
    pub(crate) fn write_into(
        &self,
        seed: [u8; 32],
        values: &mut [Value],
        target: (usize, usize, usize),
    ) -> Option<()> {
        self.write_rows(seed, values, target, ROWS)
    }
    pub(crate) fn write_frame(
        &self,
        seed: [u8; 32],
        values: &mut [Value],
        target: (usize, usize, usize),
    ) -> Option<()> {
        self.write_rows(seed, values, target, FRAME_ROWS)
    }
    fn write_rows(
        &self,
        seed: [u8; 32],
        values: &mut [Value],
        target: (usize, usize, usize),
        rows: usize,
    ) -> Option<()> {
        check_target(values.len(), target, rows)?;
        clear_frame(values, target, rows);
        self.noise.write_into(seed, values, target)?;
        let start = target.2 * target.0 + target.1;
        let root: [Value; CORE] = values[start..start + CORE].try_into().ok()?;
        let transformed = self.write_transforms(&root, values, target)?;
        write_globals(values, target, &root, &transformed, rows);
        Some(())
    }
    fn write_transforms(
        &self,
        root: &[Value; CORE],
        values: &mut [Value],
        target: (usize, usize, usize),
    ) -> Option<[[u16; 256]; 6]> {
        let mut transformed = [[0; 256]; 6];
        for (i, output) in transformed.iter_mut().enumerate() {
            let input = polynomial(root, i)?;
            let first = target.2 + NOISE_ROWS + i * NTT_ROWS;
            *output = self
                .ntt
                .write_into(input, values, (target.0, target.1 + WORK, first))?;
        }
        Some(transformed)
    }
}

fn polynomial(root: &[Value; CORE], i: usize) -> Option<[u16; 256]> {
    let mut output = [0; 256];
    for (to, from) in output
        .iter_mut()
        .zip(&root[RAW + i * 256..RAW + (i + 1) * 256])
    {
        *to = u16::try_from(from.as_canonical_u64()).ok()?;
    }
    Some(output)
}

fn check_target(len: usize, target: (usize, usize, usize), rows: usize) -> Option<()> {
    let (stride, column, first) = target;
    if stride == 0 || !len.is_multiple_of(stride) || column.checked_add(WIDTH)? > stride {
        return None;
    }
    (first.checked_add(rows)? <= len / stride).then_some(())
}

fn clear_frame(values: &mut [Value], target: (usize, usize, usize), rows: usize) {
    for row in values.chunks_exact_mut(target.0).skip(target.2).take(rows) {
        row[target.1..target.1 + WIDTH].fill(Value::ZERO);
    }
}
