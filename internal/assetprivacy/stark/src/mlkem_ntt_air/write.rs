// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{BORROW, CARRY, HIGH, LEFT, LOW, PRODUCT, QUOTIENT, RIGHT, witness::Arithmetic};
use crate::{Value, mlkem_field::Canonical};
use p3_field::PrimeCharacteristicRing;

pub(super) fn canonical(row: &mut [Value], start: usize, input: u16) {
    let value = Canonical::witness(input).expect("canonical NTT coefficient");
    row[start] = value.value;
    row[start + 1..start + 13].copy_from_slice(&value.bits);
    row[start + 13..start + 25].copy_from_slice(&value.gap);
}

pub(super) fn arithmetic(row: &mut [Value], value: &Arithmetic) {
    row[LEFT] = Value::from_u16(value.left);
    row[RIGHT] = Value::from_u16(value.right);
    for (start, input) in [
        (PRODUCT, value.product),
        (QUOTIENT, value.quotient),
        (LOW, value.low),
        (HIGH, value.high),
    ] {
        canonical(row, start, input);
    }
    row[CARRY] = Value::from_bool(value.carry);
    row[BORROW] = Value::from_bool(value.borrow);
}
