// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{KECCAK, MlKemStreamDerivationAir, ROWS, WIDTH, WINDOW, witness::Inputs};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::generate_trace_rows;

impl MlKemStreamDerivationAir {
    /// Checks key canonicality and frame bounds before changing any cell.
    pub(crate) fn write_into(
        &self,
        key: &[u8],
        message: [u8; 32],
        values: &mut [Value],
        target: (usize, usize, usize),
    ) -> Option<()> {
        let (stride, start, first) = target;
        if stride == 0
            || !values.len().is_multiple_of(stride)
            || start.checked_add(WIDTH)? > stride
            || first.checked_add(ROWS)? > values.len() / stride
        {
            return None;
        }
        let input = Inputs::new(key, message)?;
        write_rows(values, target, &input);
        Some(())
    }
}

pub(super) fn write_rows(values: &mut [Value], target: (usize, usize, usize), input: &Inputs) {
    let (stride, start, first) = target;
    for index in 0..ROWS {
        let row = &mut values[(first + index) * stride + start..][..WIDTH];
        row.fill(Value::ZERO);
        super::write::fixed(row, input, index);
    }
    let states = super::states::permutation_inputs(input);
    for (block, state) in states.into_iter().enumerate() {
        let keccak = generate_trace_rows(vec![state], 0);
        for offset in 0..24 {
            let row = &mut values[(first + block * 24 + offset) * stride + start..][..WIDTH];
            row[KECCAK..WINDOW]
                .copy_from_slice(&keccak.values[offset * KECCAK_WIDTH..][..KECCAK_WIDTH]);
        }
    }
}

const KECCAK_WIDTH: usize = WINDOW - KECCAK;
