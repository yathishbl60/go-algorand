// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Public account identities for confidential notes, never a signature substitute.
use crate::Value;
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

pub type Participants = [[u8; 32]; 4];
pub const PUBLIC_VALUES: usize = 128;

pub fn validate(participants: &Participants, inputs: usize, outputs: usize) -> Option<()> {
    if inputs > 2 || outputs > 2 {
        return None;
    }
    for (index, account) in participants.iter().enumerate() {
        let active = (index < 2 && index < inputs) || (index >= 2 && index - 2 < outputs);
        if active != (*account != [0; 32]) {
            return None;
        }
    }
    Some(())
}

pub(crate) fn append(values: &mut Vec<Value>, participants: &Participants) {
    values.extend(participants.iter().flatten().copied().map(Value::from_u8));
}

pub(crate) fn eval_public<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    participants: &Participants,
    first: usize,
) {
    for (index, &byte) in participants.iter().flatten().enumerate() {
        builder.assert_eq(builder.public_values()[first + index], Value::from_u8(byte));
    }
}

pub(crate) fn eval_account<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    flag: AB::Var,
    bits: &[AB::Var; 256],
    account: &[u8; 32],
) {
    for (bit, &value) in bits.iter().enumerate() {
        let expected = Value::from_bool(account[bit / 8] & (1 << (bit % 8)) != 0);
        builder.when(flag).assert_eq(value, expected);
    }
}

pub(crate) fn check_opening(account: &[u8; 32], opening: &[u8; 512]) -> Option<()> {
    (opening[137..169] == *account).then_some(())
}
