// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{EnvelopeAadAir, KECCAK, ROWS, WIDTH, WINDOW, witness::Inputs};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::generate_trace_rows;

impl EnvelopeAadAir {
    pub(crate) fn write_into(
        &self,
        key: &[u8],
        epoch: u64,
        ciphertext: [u8; 1088],
        values: &mut [Value],
        target: (usize, usize, usize),
    ) -> Option<[u8; 64]> {
        let (stride, start, first) = target;
        if stride == 0
            || !values.len().is_multiple_of(stride)
            || start.checked_add(WIDTH)? > stride
            || first.checked_add(ROWS)? > values.len() / stride
        {
            return None;
        }
        let input = Inputs::new(&self.context, key, epoch, ciphertext)?;
        self.write_rows(values, target, &input);
        Some(input.associated)
    }
    pub(super) fn write_rows(
        &self,
        values: &mut [Value],
        target: (usize, usize, usize),
        input: &Inputs,
    ) {
        let (stride, start, first) = target;
        for index in 0..ROWS {
            let row = &mut values[(first + index) * stride + start..][..WIDTH];
            row.fill(Value::ZERO);
            super::write::fixed(row, input, index);
        }
        write_hashes(values, target, input);
    }
}

fn write_hashes(values: &mut [Value], target: (usize, usize, usize), input: &Inputs) {
    let (stride, start, first) = target;
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
