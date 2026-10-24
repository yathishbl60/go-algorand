// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{FRAME_ROWS, MlKemEncapsulationAir, WIDTH, fixed, target, trace, writer};
use crate::{
    Value,
    mlkem_host::{Encapsulation, encapsulate},
};

impl MlKemEncapsulationAir {
    /// Writes only the complete active schedule; the parent's frame ends there.
    pub(crate) fn write_into(
        &self,
        key: &[u8],
        message: [u8; 32],
        values: &mut [Value],
        shape: (usize, usize, usize),
    ) -> Option<Encapsulation> {
        target::check(values.len(), shape, WIDTH, FRAME_ROWS)?;
        let input = encapsulate(key, message)?;
        let dots = trace::dots(&input.encryption)?;
        target::clear(values, shape, WIDTH, FRAME_ROWS);
        writer::write(key, message, &input, values, shape)?;
        fixed::write(values, &input, &dots, shape, FRAME_ROWS);
        Some(input)
    }
}
