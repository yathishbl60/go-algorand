// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    CODEC_ROW, DOT_ROW, FINAL, INV_SPAN, INVERSE_ROW, MlKemPkeAir, ROWS, WIDTH, WORK,
    prepared::Prepared, write_add, write_fixed,
};
use crate::{
    Value, mlkem_ciphertext_air::MlKemCiphertextAir, mlkem_dot_air::MlKemDotAir,
    mlkem_host::EncryptionWitness, mlkem_ntt_air::MlKemNttAir,
};
use p3_field::PrimeCharacteristicRing;

impl MlKemPkeAir {
    pub(crate) fn write_into(
        &self,
        input: &EncryptionWitness,
        values: &mut [Value],
        shape: (usize, usize, usize),
    ) -> Option<[u8; 1088]> {
        let prepared = Prepared::new(input)?;
        self.write_prepared(&prepared, values, shape)
    }

    pub(super) fn write_prepared(
        &self,
        input: &Prepared<'_>,
        values: &mut [Value],
        shape: (usize, usize, usize),
    ) -> Option<[u8; 1088]> {
        check_target(values.len(), shape)?;
        clear(values, shape);
        transforms(input, values, shape)?;
        let (stride, start, first) = shape;
        MlKemDotAir.write_into(&input.dot_input, values, (stride, start, first + DOT_ROW))?;
        write_add::write(values, input, shape);
        let ciphertext = codec(input, values, shape)?;
        write_fixed::write(values, input, shape);
        Some(ciphertext)
    }
}

fn check_target(len: usize, shape: (usize, usize, usize)) -> Option<()> {
    let (stride, start, first) = shape;
    if stride == 0 || !len.is_multiple_of(stride) || start.checked_add(WIDTH)? > stride {
        return None;
    }
    (first.checked_add(ROWS)? <= len / stride).then_some(())
}

fn clear(values: &mut [Value], shape: (usize, usize, usize)) {
    let (stride, start, first) = shape;
    for row in values.chunks_exact_mut(stride).skip(first).take(ROWS) {
        row[start..start + WIDTH].fill(Value::ZERO);
    }
}

fn transforms(
    input: &Prepared<'_>,
    values: &mut [Value],
    shape: (usize, usize, usize),
) -> Option<()> {
    let (stride, start, first) = shape;
    for slot in 0..3 {
        let target = (stride, start + WORK, first + slot * 1024);
        MlKemNttAir::forward().write_into(input.input.noise[slot], values, target)?;
    }
    for slot in 0..4 {
        let row = first + INVERSE_ROW + slot * INV_SPAN;
        MlKemNttAir::inverse().write_into(input.dots[slot], values, (stride, start + WORK, row))?;
    }
    Some(())
}

fn codec(
    input: &Prepared<'_>,
    values: &mut [Value],
    shape: (usize, usize, usize),
) -> Option<[u8; 1088]> {
    let (stride, start, first) = shape;
    MlKemCiphertextAir.write_into(
        &input.input.u,
        &input.input.v,
        values,
        (stride, start + FINAL, first + CODEC_ROW),
    )
}
