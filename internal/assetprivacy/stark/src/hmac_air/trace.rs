// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{BLOCK_BYTES, HmacAir};
use crate::{MIN_TRACE_ROWS, Value};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl HmacAir {
    pub fn trace(&self, key: &[u8; 64], data: &[u8]) -> Option<RowMajorMatrix<Value>> {
        if data.len() != self.data_len {
            return None;
        }
        let mut values = Value::zero_vec(MIN_TRACE_ROWS * self.width());
        self.write_into(key, data, &mut values, (self.width(), 0, 0))?;
        Some(RowMajorMatrix::new(values, self.width()))
    }
}

pub(super) fn padded_message(key: &[u8; 64], pad: u8, data: &[u8]) -> Vec<u8> {
    let mut message = vec![pad; BLOCK_BYTES];
    for (byte, key_byte) in key.iter().enumerate() {
        message[byte] ^= key_byte;
    }
    message.extend(data);
    message
}
