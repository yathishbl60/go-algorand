// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later
use super::{BOXES, KEY, MIX};
use crate::{aes_air::{Byte, GfMul, SBox}, aes256_air::MixColumn};

pub(super) fn byte<T: Copy>(row: &[T], offset: usize) -> Byte<T> {
    row[offset..offset + 8].try_into().expect("eight byte bits")
}

pub(super) fn key<T: Copy>(row: &[T], word: usize, byte_index: usize) -> Byte<T> {
    byte(row, KEY + word * 32 + byte_index * 8)
}

pub(super) fn sbox<T: Copy>(row: &[T], index: usize) -> SBox<T> {
    let start = BOXES + index * 241;
    SBox {
        inversion: GfMul {
            left: byte(row, start), right: byte(row, start + 8),
            powers: core::array::from_fn(|i| byte(row, start + 16 + i * 8)),
            selected: core::array::from_fn(|i| byte(row, start + 72 + i * 8)),
            sums: core::array::from_fn(|i| byte(row, start + 136 + i * 8)),
        },
        is_zero: row[start + 200],
        affine: core::array::from_fn(|i| byte(row, start + 201 + i * 8)),
        output: byte(row, start + 233),
    }
}

pub(super) fn mix<T: Copy>(row: &[T], index: usize) -> MixColumn<T> {
    let start = MIX + index * 152;
    MixColumn {
        total: core::array::from_fn(|i| byte(row, start + i * 8)),
        difference: core::array::from_fn(|i| byte(row, start + 24 + i * 8)),
        doubled: core::array::from_fn(|i| byte(row, start + 56 + i * 8)),
        partial: core::array::from_fn(|i| byte(row, start + 88 + i * 8)),
        output: core::array::from_fn(|i| byte(row, start + 120 + i * 8)),
    }
}

pub(super) fn shifted<T: Copy>(row: &[T]) -> [Byte<T>; 16] {
    core::array::from_fn(|i| byte(row, BOXES + (i % 4 + 4 * ((i / 4 + i % 4) % 4)) * 241 + 233))
}
