// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MlKemSampleAir;
use crate::Value;
use p3_air::BaseAir;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub(super) struct Prepared {
    pub candidates: Vec<u16>,
    pub fixed: Vec<Value>,
}

impl MlKemSampleAir {
    /// Rejects an exact-length prefix containing fewer than 256 accepted values.
    pub fn trace(&self, prefix: &[u8]) -> Option<RowMajorMatrix<Value>> {
        if prefix.len() != self.prefix_bytes() {
            return None;
        }
        let width = <Self as BaseAir<Value>>::width(self);
        let mut values = Value::zero_vec(self.trace_rows().checked_mul(width)?);
        self.write_into(prefix, &mut values, width, 0)?;
        Some(RowMajorMatrix::new(values, width))
    }

    pub(super) fn prepare(&self, prefix: &[u8]) -> Option<Prepared> {
        let candidates = candidates(prefix);
        let output: [u16; 256] = candidates
            .iter()
            .copied()
            .filter(|&value| value < 3329)
            .take(256)
            .collect::<Vec<_>>()
            .try_into()
            .ok()?;
        Some(Prepared {
            candidates,
            fixed: self.fixed(prefix, output),
        })
    }

    fn fixed(&self, prefix: &[u8], output: [u16; 256]) -> Vec<Value> {
        let mut fixed = Value::zero_vec(self.cursor_start());
        for (index, value) in fixed[..self.output_start()].iter_mut().enumerate() {
            *value = Value::from_bool(prefix[index / 8] & (1 << (index % 8)) != 0);
        }
        for (value, coefficient) in fixed[self.output_start()..].iter_mut().zip(output) {
            *value = Value::from_u16(coefficient);
        }
        fixed
    }
}

fn candidates(prefix: &[u8]) -> Vec<u16> {
    prefix
        .as_chunks::<3>()
        .0
        .iter()
        .flat_map(|bytes| {
            let low = u16::from(bytes[0]) + (u16::from(bytes[1] & 15) << 8);
            let high = u16::from(bytes[1] >> 4) + (u16::from(bytes[2]) << 4);
            [low, high]
        })
        .collect()
}
