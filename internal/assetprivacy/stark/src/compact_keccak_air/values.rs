// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CompactKeccakAir, PREPROCESSED, ProgramColumn as P, WIDTH, WORDS};
use crate::serial_keccak::{RC_POSITIONS, parity_index, rho_source, state_index};
use p3_field::{Algebra, PrimeCharacteristicRing, PrimeField64};

impl<F: PrimeField64> CompactKeccakAir<F> {
    /// Little-endian uint16 of the reconstructed private pre-Theta input.
    pub fn input_word<E: Algebra<F>, T: Copy + Into<E>>(
        &self,
        row: &[T],
        word: usize,
    ) -> Option<E> {
        (row.len() == WIDTH && word < WORDS).then(|| {
            (0..16).rev().fold(E::ZERO, |v, bit| {
                v.double() + input_bit::<E, T>(row, word * 16 + bit)
            })
        })
    }

    /// Program must be this AIR's trusted preprocessing row, never witness data.
    pub fn output_word<E: Algebra<F>, T: Copy + Into<E>>(
        &self,
        row: &[T],
        program: &[T],
        word: usize,
    ) -> Option<E> {
        (row.len() == WIDTH && program.len() == PREPROCESSED && word < WORDS).then(|| {
            (0..16).rev().fold(E::ZERO, |v, bit| {
                v.double() + output_bit::<E, T>(row, program, word * 16 + bit)
            })
        })
    }
}

pub(super) fn xor3<E: PrimeCharacteristicRing, T: Copy + Into<E>>(bits: [T; 3]) -> E {
    let first: E = bits[0].into();
    first.xor(&bits[1].into()).xor(&bits[2].into())
}

pub(super) fn input_bit<E: PrimeCharacteristicRing, T: Copy + Into<E>>(
    row: &[T],
    index: usize,
) -> E {
    let (x, z) = ((index / 64) % 5, index % 64);
    xor3([
        row[index],
        row[parity_index((x + 4) % 5, z)],
        row[parity_index((x + 1) % 5, (z + 63) % 64)],
    ])
}

pub(super) fn output_bit<E: PrimeCharacteristicRing, T: Copy + Into<E>>(
    row: &[T],
    program: &[T],
    index: usize,
) -> E {
    let (x, y, z) = ((index / 64) % 5, index / 320, index % 64);
    let a: E = row[rho_source(index)].into();
    let b: E = row[rho_source(state_index((x + 1) % 5, y, z))].into();
    let c: E = row[rho_source(state_index((x + 2) % 5, y, z))].into();
    let chi = a.xor(&((E::ONE - b) * c));
    if index >= 64 {
        return chi;
    }
    match RC_POSITIONS.iter().position(|&bit| bit == z) {
        Some(column) => chi.xor(&program[P::RoundConstant0 as usize + column].into()),
        None => chi,
    }
}
