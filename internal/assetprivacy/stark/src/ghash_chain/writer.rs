// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    ACTIVE_ROWS, BITS, GhashChainAir, WIDTH, trace::Inputs, witness_state::State, witness_write,
};
use crate::Value;

impl GhashChainAir {
    pub(crate) fn write_into(
        &self,
        h: [u8; 16],
        associated: [u8; 64],
        ciphertext: [u8; 512],
        values: &mut [Value],
        layout: (usize, usize, usize),
    ) -> Option<[u8; 16]> {
        check_target(values.len(), layout)?;
        let input = Inputs {
            h: u128::from_be_bytes(h),
            associated,
            ciphertext,
        };
        Some(write(&input, values, layout))
    }
}

fn check_target(len: usize, layout: (usize, usize, usize)) -> Option<()> {
    let (width, first, start) = layout;
    if width == 0 || !len.is_multiple_of(width) || start.checked_add(WIDTH)? > width {
        return None;
    }
    if first.checked_add(ACTIVE_ROWS + 1)? > len / width {
        return None;
    }
    Some(())
}

fn write(input: &Inputs, values: &mut [Value], layout: (usize, usize, usize)) -> [u8; 16] {
    let (width, first, start) = layout;
    let result = input.result();
    let mut state = State::new();
    for (round, target) in values.chunks_exact_mut(width).skip(first).enumerate() {
        let row = &mut target[start..start + WIDTH];
        input.prepare(&mut state, round);
        witness_write::fixed(row, input.h, &input.associated, result);
        witness_write::state(row, &state, round);
        if round < ACTIVE_ROWS {
            state.advance(round % BITS);
        }
    }
    result.to_be_bytes()
}
