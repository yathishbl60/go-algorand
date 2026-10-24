// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemNoiseAir, trace::output, witness::auxiliary};
use crate::{Value, sha3_air::Sha3Air};
use p3_air::BaseAir;

impl MlKemNoiseAir {
    pub(crate) fn write_into(
        &self,
        seed: [u8; 32],
        values: &mut [Value],
        shape: (usize, usize, usize),
    ) -> Option<()> {
        let (stride, start, first) = shape;
        self.check_target(values.len(), shape)?;
        let target = &mut values[first * stride..(first + self.trace_rows()) * stride];
        let message = self.message(seed);
        let hash = Sha3Air::shake256_128(message.len())?;
        hash.write_into(&message, target, stride, start)?;
        let cbd = auxiliary(output(&message));
        for row in target.chunks_exact_mut(stride) {
            row[start + self.digest_start()..start + self.width()].copy_from_slice(&cbd);
        }
        Some(())
    }

    fn check_target(&self, len: usize, shape: (usize, usize, usize)) -> Option<()> {
        let (stride, start, first) = shape;
        if stride == 0 || !len.is_multiple_of(stride) || start.checked_add(self.width())? > stride {
            return None;
        }
        (first.checked_add(self.trace_rows())? <= len / stride).then_some(())
    }
}
