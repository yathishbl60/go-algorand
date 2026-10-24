// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AesGcmAir, ROWS, WIDTH, witness::Fixed};
use crate::Value;

pub(crate) struct Encrypted {
    pub hash: [u8; 16],
    pub mask: [u8; 16],
    pub ciphertext: [u8; 512],
}

impl AesGcmAir {
    pub(crate) fn write_into(
        &self,
        key: [u8; 32],
        nonce: [u8; 12],
        plaintext: [u8; 512],
        values: &mut [Value],
        width: usize,
    ) -> Option<Encrypted> {
        if width == 0 {
            return None;
        }
        let shape = (width, 0, 0, values.len() / width);
        self.write_frame(key, nonce, plaintext, values, shape)
    }

    pub(crate) fn write_frame(
        &self,
        key: [u8; 32],
        nonce: [u8; 12],
        plaintext: [u8; 512],
        values: &mut [Value],
        shape: (usize, usize, usize, usize),
    ) -> Option<Encrypted> {
        check_target(values.len(), shape)?;
        let fixed = Fixed::new(key, nonce, plaintext);
        super::write_frame::initialize(&fixed, values, shape);
        Some(Encrypted {
            hash: fixed.hash.map(super::write_frame::byte),
            mask: fixed.mask.map(super::write_frame::byte),
            ciphertext: super::write_frame::blocks(&fixed, values, shape),
        })
    }
}

fn check_target(len: usize, shape: (usize, usize, usize, usize)) -> Option<()> {
    let (stride, start, first, rows) = shape;
    if stride == 0 || !len.is_multiple_of(stride) || start.checked_add(WIDTH)? > stride {
        return None;
    }
    (rows >= ROWS && first.checked_add(rows)? <= len / stride).then_some(())
}
