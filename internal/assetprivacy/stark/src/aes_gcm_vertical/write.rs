// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later
use crate::{
    Value,
    aes_air::{Byte, SBox},
    aes256_air::MixColumn,
};
use p3_field::PrimeCharacteristicRing;

pub(super) fn bytes(row: &mut [Value], values: &[Byte<Value>]) {
    for (target, source) in row.as_chunks_mut::<8>().0.iter_mut().zip(values) {
        target.copy_from_slice(source);
    }
}

pub(super) fn raw(row: &mut [Value], values: &[u8]) {
    for (target, &source) in row.as_chunks_mut::<8>().0.iter_mut().zip(values) {
        for (bit, value) in target.iter_mut().enumerate() {
            *value = Value::from_bool(source & (1 << bit) != 0);
        }
    }
}

pub(super) fn sbox(row: &mut [Value], value: &SBox<Value>) {
    bytes(&mut row[..8], &[value.inversion.left]);
    bytes(&mut row[8..16], &[value.inversion.right]);
    bytes(&mut row[16..72], &value.inversion.powers);
    bytes(&mut row[72..136], &value.inversion.selected);
    bytes(&mut row[136..200], &value.inversion.sums);
    row[200] = value.is_zero;
    bytes(&mut row[201..233], &value.affine);
    bytes(&mut row[233..241], &[value.output]);
}

pub(super) fn mix(row: &mut [Value], value: &MixColumn<Value>) {
    bytes(&mut row[..24], &value.total);
    bytes(&mut row[24..56], &value.difference);
    bytes(&mut row[56..88], &value.doubled);
    bytes(&mut row[88..120], &value.partial);
    bytes(&mut row[120..152], &value.output);
}
