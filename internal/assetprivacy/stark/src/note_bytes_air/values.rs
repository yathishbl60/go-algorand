// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{BYTE_BITS, NoteBytesAir, PREVIOUS, WIDTH};
use p3_field::{Algebra, PrimeCharacteristicRing, PrimeField64};

impl<F: PrimeField64> NoteBytesAir<F> {
    /// Fixed linear byte expression; this does not authenticate a bus lookup.
    pub fn byte_value<E: Algebra<F>, T: Copy + Into<E>>(&self, row: &[T]) -> Option<E> {
        (row.len() == WIDTH).then(|| byte(&row[..BYTE_BITS]))
    }

    /// Little-endian overlap word. Trusted WordActive must disable note starts.
    pub fn overlapping_word<E: Algebra<F>, T: Copy + Into<E>>(&self, row: &[T]) -> Option<E> {
        self.byte_value::<E, T>(row)
            .map(|current| row[PREVIOUS].into() + current * F::from_u16(256))
    }
}

// Constant multiplication keeps recursive symbolic lookup resolution linear.
pub(super) fn byte<E: PrimeCharacteristicRing, T: Copy + Into<E>>(bits: &[T]) -> E {
    bits.iter()
        .rev()
        .fold(E::ZERO, |value, &bit| value * E::TWO + bit.into())
}

pub(super) fn count<E: PrimeCharacteristicRing, T: Copy + Into<E>>(bits: &[T]) -> E {
    bits.iter().map(|&bit| bit.into()).sum()
}
