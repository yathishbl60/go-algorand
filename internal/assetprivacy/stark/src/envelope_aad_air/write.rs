// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CANONICAL, EPOCH, KECCAK, OUTPUT, RHO, WINDOW, message, witness::Inputs};
use crate::{Value, mlkem_field::Canonical};
use p3_field::PrimeCharacteristicRing;

pub(super) fn fixed(row: &mut [Value], input: &Inputs, index: usize) {
    for (target, &coefficient) in row[..RHO].iter_mut().zip(&input.polynomials) {
        *target = Value::from_u16(coefficient);
    }
    bits(&mut row[RHO..OUTPUT], &input.key[1152..]);
    bits(&mut row[OUTPUT..EPOCH], &input.associated);
    bits(&mut row[EPOCH..KECCAK], &input.context[147..155]);
    let selected = input.polynomials.get(index).copied().unwrap_or(0);
    let canonical = Canonical::witness(selected).expect("checked canonical key");
    row[CANONICAL] = canonical.value;
    row[CANONICAL + 1..CANONICAL + 13].copy_from_slice(&canonical.bits);
    row[CANONICAL + 13..CANONICAL + 25].copy_from_slice(&canonical.gap);
    window(row, input, index);
}

fn window(row: &mut [Value], input: &Inputs, index: usize) {
    let block = index / 24;
    if index < 864 && (3..20).contains(&block) {
        let start = message::key_start(block);
        for bit in 0..588 {
            let global = start + bit;
            if global < 9472 {
                row[WINDOW + bit] =
                    Value::from_bool(input.key[global / 8] & (1 << (global % 8)) != 0);
            }
        }
    } else if index < 864 && block >= 20 {
        let start = (block - 20) * 72;
        let count = (1088 - start).min(72);
        bits(
            &mut row[WINDOW..WINDOW + count * 8],
            &input.ciphertext[start..start + count],
        );
    }
}

fn bits(target: &mut [Value], source: &[u8]) {
    for (index, bit) in target.iter_mut().enumerate() {
        *bit = Value::from_bool(source[index / 8] & (1 << (index % 8)) != 0);
    }
}
