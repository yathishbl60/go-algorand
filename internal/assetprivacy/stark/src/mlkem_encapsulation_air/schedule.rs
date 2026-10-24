// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    FRAME_ROWS, MATRIX_ROW, MlKemEncapsulationAir, NOISE_ROW, PKE_ROW, PREPROCESSED, ROWS,
    schedule_write::{copy_ref, flags},
    target,
};
use crate::{
    Value, mlkem_pke_air::MlKemPkeAir, mlkem_stream_derivation_air::MlKemStreamDerivationAir,
    mlkem_stream_matrix_air::MlKemStreamMatrixAir,
};
use p3_air::BaseAir;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub(super) fn build() -> RowMajorMatrix<Value> {
    let mut values = Value::zero_vec(ROWS * PREPROCESSED);
    populate(&mut values, (PREPROCESSED, 0, 0));
    RowMajorMatrix::new(values, PREPROCESSED)
}

impl MlKemEncapsulationAir {
    /// Writes trusted active-frame preprocessing without a full KEM table copy.
    pub(crate) fn write_preprocessed_into(
        &self,
        values: &mut [Value],
        shape: (usize, usize, usize),
    ) -> Option<()> {
        target::check(values.len(), shape, PREPROCESSED, FRAME_ROWS)?;
        target::clear(values, shape, PREPROCESSED, FRAME_ROWS);
        populate(values, shape);
        Some(())
    }
}

fn populate(values: &mut [Value], shape: (usize, usize, usize)) {
    let key = MlKemStreamDerivationAir.preprocessed_trace().expect("key");
    copy_ref(values, shape, 0, 0, &key);
    matrices(values, shape);
    for index in 0..7 {
        flags(values, shape, NOISE_ROW + 512 * index, 10 + index, 512);
    }
    let pke = MlKemPkeAir.preprocessed_trace().expect("PKE");
    copy_ref(values, shape, PKE_ROW, 17, &pke);
}

fn matrices(values: &mut [Value], shape: (usize, usize, usize)) {
    let air = MlKemStreamMatrixAir::new(0, 0).expect("coordinate");
    let matrix = air.preprocessed_trace().expect("matrix");
    for index in 0..9 {
        copy_ref(values, shape, MATRIX_ROW + 4096 * index, 1 + index, &matrix);
    }
}
