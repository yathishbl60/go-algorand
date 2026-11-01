// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    BIT_COUNT, BYTE_BITS, ENCODED_SIZE, END_INVERSE, NoteBytesAir, PREVIOUS, WIDTH, validation,
};
use p3_field::PrimeField64;
use p3_matrix::dense::RowMajorMatrix;

impl<F: PrimeField64> NoteBytesAir<F> {
    /// Validates every opening before allocating/writing a private trace.
    pub fn trace(&self, openings: &[[u8; ENCODED_SIZE]]) -> Option<RowMajorMatrix<F>> {
        if openings.len() != self.count
            || !openings.iter().all(|opening| self.valid_opening(opening))
        {
            return None;
        }
        let mut values = Vec::new();
        values.try_reserve_exact(self.rows() * WIDTH).ok()?;
        values.resize(self.rows() * WIDTH, F::ZERO);
        for (note, opening) in openings.iter().enumerate() {
            let start = note * ENCODED_SIZE * WIDTH;
            self.write_note(opening, &mut values[start..start + ENCODED_SIZE * WIDTH])?;
        }
        Some(RowMajorMatrix::new(values, WIDTH))
    }

    fn write_note(&self, opening: &[u8; ENCODED_SIZE], values: &mut [F]) -> Option<()> {
        let (mut previous, mut sum) = (0u8, 0u32);
        for (offset, &byte) in opening.iter().enumerate() {
            let row = &mut values[offset * WIDTH..(offset + 1) * WIDTH];
            for (bit, cell) in row[..BYTE_BITS].iter_mut().enumerate() {
                *cell = F::from_bool(byte & (1 << bit) != 0);
            }
            row[PREVIOUS] = F::from_u8(previous);
            row[BIT_COUNT] = F::from_u32(sum);
            sum = finish_byte(row, byte, sum, offset)?;
            previous = byte;
        }
        Some(())
    }
}

fn finish_byte<F: PrimeField64>(row: &mut [F], byte: u8, sum: u32, offset: usize) -> Option<u32> {
    let count = byte.count_ones();
    if validation::nonzero_end(offset) {
        row[END_INVERSE] = F::from_u32(sum + count).try_inverse()?;
        Some(0)
    } else {
        Some(sum + u32::from(validation::nonzero_active(offset)) * count)
    }
}
