// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    A0, A1, B0, B1, C0, C1, CARRY0, CARRY1, GAMMA, GP11, P00, P01, P10, P11, witness::Pair,
};
use crate::{Value, mlkem_field::Canonical};
use p3_field::PrimeCharacteristicRing;

pub(crate) fn canonical(row: &mut [Value], start: usize, input: u16) {
    let value = Canonical::witness(input).expect("canonical NTT product coefficient");
    row[start] = value.value;
    row[start + 1..start + 13].copy_from_slice(&value.bits);
    row[start + 13..start + 25].copy_from_slice(&value.gap);
}

pub(crate) fn pair(row: &mut [Value], value: &Pair) {
    for (start, input) in [A0, A1, B0, B1].into_iter().zip(value.operands) {
        canonical(row, start, input);
    }
    canonical(row, GAMMA, value.gamma);
    for (start, (output, quotient)) in [P11, GP11, P00, P01, P10].into_iter().zip(value.products) {
        canonical(row, start, output);
        canonical(row, start + 25, quotient);
    }
    for ((start, carry), (output, overflow)) in
        [(C0, CARRY0), (C1, CARRY1)].into_iter().zip(value.sums)
    {
        canonical(row, start, output);
        row[carry] = Value::from_bool(overflow);
    }
}
