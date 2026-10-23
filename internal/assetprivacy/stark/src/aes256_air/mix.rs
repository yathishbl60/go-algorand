// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::byte::eval_xor;
use crate::{
    Value,
    aes_air::{Byte, eval_byte, eval_xtime},
};
use p3_air::AirBuilder;

/// 152 columns. Inputs supplied by the caller are the ShiftRows column.
pub struct MixColumn<T> {
    pub total: [Byte<T>; 3],
    pub difference: [Byte<T>; 4],
    pub doubled: [Byte<T>; 4],
    pub partial: [Byte<T>; 4],
    pub output: [Byte<T>; 4],
}

pub fn eval_mix<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    input: [Byte<AB::Var>; 4],
    value: &MixColumn<AB::Var>,
) {
    for byte in input {
        eval_byte(builder, byte);
    }
    eval_xor(builder, input[0], input[1], value.total[0]);
    eval_xor(builder, value.total[0], input[2], value.total[1]);
    eval_xor(builder, value.total[1], input[3], value.total[2]);
    for row in 0..4 {
        eval_row(builder, input, value, row);
    }
    eval_ranges(builder, value);
}

fn eval_row<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    input: [Byte<AB::Var>; 4],
    value: &MixColumn<AB::Var>,
    row: usize,
) {
    eval_xor(
        builder,
        input[row],
        input[(row + 1) % 4],
        value.difference[row],
    );
    eval_xtime(builder, value.difference[row], value.doubled[row]);
    eval_xor(builder, input[row], value.total[2], value.partial[row]);
    let partial = value.partial[row];
    eval_xor(builder, partial, value.doubled[row], value.output[row]);
}

fn eval_ranges<AB: AirBuilder<F = Value>>(builder: &mut AB, value: &MixColumn<AB::Var>) {
    for byte in value.total {
        eval_byte(builder, byte);
    }
    for byte in value.partial {
        eval_byte(builder, byte);
    }
    for byte in value.output {
        eval_byte(builder, byte);
    }
}
