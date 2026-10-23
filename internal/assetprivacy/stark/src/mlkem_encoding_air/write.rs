// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{LOCAL_BITS, REMAINDER, ROLLOVER};
use crate::{
    Value,
    mlkem_field::{Canonical, Compression},
};
use p3_field::PrimeCharacteristicRing;

pub(super) fn canonical(row: &mut [Value], start: usize, input: u16) {
    let value = Canonical::witness(input).expect("canonical encoding coefficient");
    copy_canonical(row, start, value);
}

fn copy_canonical(row: &mut [Value], start: usize, value: Canonical<Value>) {
    row[start] = value.value;
    row[start + 1..start + 13].copy_from_slice(&value.bits);
    row[start + 13..start + 25].copy_from_slice(&value.gap);
}

pub(super) fn arithmetic(row: &mut [Value], bits: usize, input: u16) {
    canonical(row, 0, input);
    match bits {
        4 => compression::<4>(row, input),
        10 => compression::<10>(row, input),
        _ => {
            let output = encoded(input, bits);
            for bit in 0..bits {
                row[LOCAL_BITS + bit] = Value::from_bool(output & (1 << bit) != 0);
            }
        }
    }
}

fn compression<const D: usize>(row: &mut [Value], input: u16) {
    let value = Compression::<Value, D>::witness(input).expect("valid compression");
    row[LOCAL_BITS..LOCAL_BITS + D].copy_from_slice(&value.compressed);
    row[ROLLOVER] = value.rollover;
    copy_canonical(row, REMAINDER, value.remainder);
}

pub(super) fn encoded(input: u16, bits: usize) -> u16 {
    match bits {
        1 => u16::from(input != 0),
        12 => input,
        _ => ((((u32::from(input) << bits) + 1664) / 3329) % (1 << bits)) as u16,
    }
}
