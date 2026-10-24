// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AAD_ROW, CHILD_PRE, EnvelopeAir, GCM_ROW, HKDF_ROW, NOTE_ROW, PREPROCESSED, ROWS};
use crate::{Value, gcm_vertical::GcmVerticalAir, mlkem_encapsulation_air::MlKemEncapsulationAir};
use p3_air::BaseAir;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::{Matrix, dense::RowMajorMatrix};

pub(super) fn build(air: &EnvelopeAir) -> RowMajorMatrix<Value> {
    let mut values = Value::zero_vec(ROWS * PREPROCESSED);
    MlKemEncapsulationAir
        .write_preprocessed_into(&mut values, (PREPROCESSED, 0, 0))
        .expect("bounded KEM schedule");
    flags(&mut values, 0, 0, AAD_ROW);
    let aad = air.aad.preprocessed_trace().expect("AAD");
    copy(&mut values, AAD_ROW, 1, aad);
    flags(&mut values, HKDF_ROW, 2, 512);
    let gcm = GcmVerticalAir.frame_preprocessed_trace();
    copy(&mut values, GCM_ROW, 3, gcm);
    flags(&mut values, NOTE_ROW, 4, 512);
    RowMajorMatrix::new(values, PREPROCESSED)
}

fn copy(values: &mut [Value], first: usize, stage: usize, table: RowMajorMatrix<Value>) {
    assert!(
        table.width() <= CHILD_PRE,
        "fixed envelope child preprocessing"
    );
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
