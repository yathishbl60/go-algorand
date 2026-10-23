// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{BLOCK_BYTES, HmacAir};
use crate::{MIN_TRACE_ROWS, Value};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;
use sha3::{Digest, Sha3_512};

impl HmacAir {
    pub fn trace(&self, key: &[u8; 64], data: &[u8]) -> Option<RowMajorMatrix<Value>> {
        if data.len() != self.data_len {
            return None;
        }
        let inner = padded_message(key, 0x36, data);
        let inner_digest: [u8; 64] = Sha3_512::digest(&inner).into();
        let outer = padded_message(key, 0x5c, &inner_digest);
        let output: [u8; 64] = Sha3_512::digest(&outer).into();
        let mut trace =
            RowMajorMatrix::new(Value::zero_vec(MIN_TRACE_ROWS * self.width()), self.width());
        self.copy_hashes(&mut trace, &inner, &outer)?;
        self.write_auxiliary(&mut trace, key, inner_digest, output);
        Some(trace)
    }
}

fn padded_message(key: &[u8; 64], pad: u8, data: &[u8]) -> Vec<u8> {
    let mut message = vec![pad; BLOCK_BYTES];
    for (byte, key_byte) in key.iter().enumerate() {
        message[byte] ^= key_byte;
    }
    message.extend(data);
    message
}
