// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{HkdfAir, INFO};
use crate::{MIN_TRACE_ROWS, Value, hmac_air::derive_output};

pub(crate) fn derive_key(shared: &[u8; 32], salt: &[u8; 64]) -> [u8; 32] {
    let prk = derive_output(salt, shared);
    let output = derive_output(&prk, &info());
    core::array::from_fn(|index| output[index])
}

impl HkdfAir {
    pub(crate) fn write_into(
        &self,
        shared: &[u8; 32],
        salt: &[u8; 64],
        values: &mut [Value],
        shape: (usize, usize, usize),
    ) -> Option<[u8; 32]> {
        self.check_target(values.len(), shape)?;
        let prk = self.extract.write_into(salt, shared, values, shape)?;
        let (stride, start, first) = shape;
        let target = (stride, start + self.extract.width(), first);
        let output = self.expand.write_into(&prk, &info(), values, target)?;
        Some(core::array::from_fn(|index| output[index]))
    }

    fn check_target(&self, len: usize, shape: (usize, usize, usize)) -> Option<()> {
        let (stride, start, first) = shape;
        if stride == 0 || !len.is_multiple_of(stride) || start.checked_add(self.width())? > stride {
            return None;
        }
        (first.checked_add(MIN_TRACE_ROWS)? <= len / stride).then_some(())
    }
}

fn info() -> Vec<u8> {
    INFO.iter().copied().chain(core::iter::once(1)).collect()
}
