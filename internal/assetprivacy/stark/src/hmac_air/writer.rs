// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{HmacAir, prepared::Prepared};
use crate::{MIN_TRACE_ROWS, Value};

impl HmacAir {
    pub(crate) fn write_into(
        &self,
        key: &[u8; 64],
        data: &[u8],
        values: &mut [Value],
        shape: (usize, usize, usize),
    ) -> Option<[u8; 64]> {
        self.check_target(data.len(), values.len(), shape)?;
        let input = Prepared::new(key, data);
        let (stride, start, first) = shape;
        let target = &mut values[first * stride..(first + MIN_TRACE_ROWS) * stride];
        self.inner.write_into(&input.inner, target, stride, start)?;
        self.outer
            .write_into(&input.outer, target, stride, start + self.inner.width())?;
        self.write_auxiliary(target, stride, start, key, &input);
        Some(input.output)
    }

    fn check_target(&self, bytes: usize, len: usize, shape: (usize, usize, usize)) -> Option<()> {
        let (stride, start, first) = shape;
        if bytes != self.data_len || stride == 0 || !len.is_multiple_of(stride) {
            return None;
        }
        if start.checked_add(self.width())? > stride {
            return None;
        }
        (first.checked_add(MIN_TRACE_ROWS)? <= len / stride).then_some(())
    }
}
