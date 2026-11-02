// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CONTENT_SIZE, ENCODED_SIZE, NONZERO, NoteBytesAir, PREFIX_SIZE};
use p3_field::PrimeField64;

impl<F: PrimeField64> NoteBytesAir<F> {
    pub(super) fn expected_byte(&self, offset: usize) -> Option<u8> {
        if offset < PREFIX_SIZE {
            Some(self.prefix[offset])
        } else {
            (CONTENT_SIZE..ENCODED_SIZE).contains(&offset).then_some(0)
        }
    }

    pub(crate) fn valid_opening(&self, opening: &[u8; ENCODED_SIZE]) -> bool {
        opening[..PREFIX_SIZE] == self.prefix
            && opening[CONTENT_SIZE..].iter().all(|&byte| byte == 0)
            && NONZERO
                .iter()
                .all(|&(start, end)| opening[start..end].iter().any(|&byte| byte != 0))
    }
}

pub(super) fn nonzero_active(offset: usize) -> bool {
    NONZERO
        .iter()
        .any(|&(start, end)| (start..end).contains(&offset))
}

pub(super) fn nonzero_end(offset: usize) -> bool {
    NONZERO.iter().any(|&(_, end)| offset + 1 == end)
}
