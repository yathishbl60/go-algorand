// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CHILD_PRE, MATRIX_ROW, NOISE_ROW, PKE_ROW, PREPROCESSED, ROWS};
use crate::{
    Value, mlkem_pke_air::MlKemPkeAir, mlkem_stream_derivation_air::MlKemStreamDerivationAir,
    mlkem_stream_matrix_air::MlKemStreamMatrixAir,
};
use p3_air::BaseAir;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::{Matrix, dense::RowMajorMatrix};

pub(super) fn build() -> RowMajorMatrix<Value> {
    let mut values = Value::zero_vec(ROWS * PREPROCESSED);
    let key = MlKemStreamDerivationAir.preprocessed_trace().expect("key");
    copy(&mut values, 0, 0, key);
    matrices(&mut values);
    for index in 0..7 {
        flags(&mut values, NOISE_ROW + 512 * index, 10 + index, 512);
    }
    let pke = MlKemPkeAir.preprocessed_trace().expect("PKE");
    copy(&mut values, PKE_ROW, 17, pke);
    RowMajorMatrix::new(values, PREPROCESSED)
}

fn matrices(values: &mut [Value]) {
    let air = MlKemStreamMatrixAir::new(0, 0).expect("coordinate");
    let matrix = air.preprocessed_trace().expect("matrix");
    for index in 0..9 {
        copy_ref(values, MATRIX_ROW + 4096 * index, 1 + index, &matrix);
    }
}

fn copy(values: &mut [Value], first: usize, stage: usize, table: RowMajorMatrix<Value>) {
    copy_ref(values, first, stage, &table);
}

fn copy_ref(values: &mut [Value], first: usize, stage: usize, table: &RowMajorMatrix<Value>) {
    assert!(table.width() <= CHILD_PRE, "fixed child preprocessing");
    for local in 0..table.height() {
        let row = &mut values[(first + local) * PREPROCESSED..][..PREPROCESSED];
        row[..table.width()]
            .copy_from_slice(&table.values[local * table.width()..][..table.width()]);
    }
    flags(values, first, stage, table.height());
}

fn flags(values: &mut [Value], first: usize, stage: usize, height: usize) {
    for local in 0..height {
        let row = &mut values[(first + local) * PREPROCESSED..][..PREPROCESSED];
        let start = CHILD_PRE + 4 * stage;
        row[start] = Value::ONE;
        row[start + 1] = Value::from_bool(local == 0);
        row[start + 2] = Value::from_bool(local + 1 == height);
        row[start + 3] = Value::from_bool(local + 1 < height);
    }
}
