// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CANONICAL, HASH, MESSAGE, OUTPUT, RHO, WINDOW, witness::Inputs};
use crate::{Value, mlkem_field::Canonical};
use p3_field::PrimeCharacteristicRing;

pub(super) fn fixed(row: &mut [Value], input: &Inputs, index: usize) {
    for (target, &coefficient) in row[..RHO].iter_mut().zip(&input.polynomials) {
        *target = Value::from_u16(coefficient);
    }
    bits(&mut row[RHO..MESSAGE], &input.key[1152..]);
    bits(&mut row[MESSAGE..HASH + 256], &input.message);
    bits(&mut row[OUTPUT..OUTPUT + 512], &input.output);
    let selected = if index < 768 {
        input.polynomials[index]
    } else {
        0
    };
    let canonical = Canonical::witness(selected).expect("checked canonical key");
    row[CANONICAL] = canonical.value;
    row[CANONICAL + 1..CANONICAL + 13].copy_from_slice(&canonical.bits);
    row[CANONICAL + 13..CANONICAL + 25].copy_from_slice(&canonical.gap);
    window(row, input, index);
}

fn window(row: &mut [Value], input: &Inputs, index: usize) {
    if index >= 216 {
        return;
    }
    let start = (index / 24) * 1088 / 12 * 12;
    for bit in 0..1104 {
        let global = start + bit;
        if global < input.key.len() * 8 {
            row[WINDOW + bit] = Value::from_bool(input.key[global / 8] & (1 << (global % 8)) != 0);
        }
    }
}

fn bits(target: &mut [Value], source: &[u8]) {
    for (index, bit) in target.iter_mut().enumerate() {
        *bit = Value::from_bool(source[index / 8] & (1 << (index % 8)) != 0);
    }
}
