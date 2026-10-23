// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::HmacAir;
use crate::{MIN_TRACE_ROWS, Value, sha3_air::digest_values};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::{Matrix, dense::RowMajorMatrix};

impl HmacAir {
    pub(super) fn copy_hashes(
        &self,
        trace: &mut RowMajorMatrix<Value>,
        inner: &[u8],
        outer: &[u8],
    ) -> Option<()> {
        let mut start = 0;
        for (component, message) in [(&self.inner, inner), (&self.outer, outer)] {
            let hash = component.trace(message)?;
            if hash.height() != MIN_TRACE_ROWS {
                return None;
            }
            let width = component.width();
            for (r, row) in trace.values.chunks_exact_mut(self.width()).enumerate() {
                row[start..start + width].copy_from_slice(&hash.values[r * width..(r + 1) * width]);
            }
            start += width;
        }
        Some(())
    }

    pub(super) fn write_auxiliary(
        &self,
        trace: &mut RowMajorMatrix<Value>,
        key: &[u8; 64],
        inner: [u8; 64],
        outer: [u8; 64],
    ) {
        for row in trace.values.chunks_exact_mut(self.width()) {
            let start = self.digest_start();
            row[start..start + 32].copy_from_slice(&digest_values(inner));
            row[start + 32..start + 64].copy_from_slice(&digest_values(outer));
            self.write_bits(row, key, &outer);
        }
    }

    fn write_bits(&self, row: &mut [Value], key: &[u8; 64], output: &[u8; 64]) {
        for byte in 0..64 {
            for bit in 0..8 {
                let index = byte * 8 + bit;
                row[self.key_start() + index] = Value::from_bool(key[byte] & (1 << bit) != 0);
                row[self.output_start() + index] = Value::from_bool(output[byte] & (1 << bit) != 0);
            }
        }
    }
}
