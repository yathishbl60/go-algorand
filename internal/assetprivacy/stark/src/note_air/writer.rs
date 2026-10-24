// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ENCODED_SIZE, NoteCommitmentAir, trace::nonzero_inverses};
use crate::{MIN_TRACE_ROWS, Value};
use sha3::{Digest, Sha3_512};

impl NoteCommitmentAir {
    pub(crate) fn write_into(
        &self,
        opening: &[u8; ENCODED_SIZE],
        values: &mut [Value],
        shape: (usize, usize, usize),
    ) -> Option<[u8; 64]> {
        self.check_target(values.len(), shape)?;
        let message = self.message(opening)?;
        let inverses = nonzero_inverses(opening);
        let (stride, start, first) = shape;
        let target = &mut values[first * stride..(first + MIN_TRACE_ROWS) * stride];
        self.hash.write_into(&message, target, stride, start)?;
        for row in target.chunks_exact_mut(stride) {
            row[start + self.hash.width()..start + self.width()].copy_from_slice(&inverses);
        }
        Some(Sha3_512::digest(message).into())
    }

    fn check_target(&self, len: usize, shape: (usize, usize, usize)) -> Option<()> {
        let (stride, start, first) = shape;
        if stride == 0 || !len.is_multiple_of(stride) || start.checked_add(self.width())? > stride {
            return None;
        }
        (first.checked_add(MIN_TRACE_ROWS)? <= len / stride).then_some(())
    }
}
