// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    AAD_ROW, EnvelopeAir, FRAME_ROWS, GCM_ROW, HKDF_ROW, NOTE_ROW, PREPROCESSED, ROWS,
    schedule_copy::{copy, flags},
    target,
};
use crate::{Value, gcm_vertical::GcmVerticalAir, mlkem_encapsulation_air::MlKemEncapsulationAir};
use p3_air::BaseAir;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub(super) fn build(air: &EnvelopeAir) -> RowMajorMatrix<Value> {
    let mut values = Value::zero_vec(ROWS * PREPROCESSED);
    populate(air, &mut values, (PREPROCESSED, 0, 0)).expect("fixed envelope schedule");
    RowMajorMatrix::new(values, PREPROCESSED)
}

impl EnvelopeAir {
    /// Writes only the complete active schedule into a bounded parent rectangle.
    pub(crate) fn write_preprocessed_into(
        &self,
        values: &mut [Value],
        shape: (usize, usize, usize),
    ) -> Option<()> {
        target::check(values.len(), shape, PREPROCESSED, FRAME_ROWS)?;
        target::clear(values, shape, PREPROCESSED, FRAME_ROWS);
        populate(self, values, shape)
    }
}

fn populate(air: &EnvelopeAir, values: &mut [Value], shape: (usize, usize, usize)) -> Option<()> {
    MlKemEncapsulationAir.write_preprocessed_into(values, shape)?;
    flags(values, shape, 0, 0, AAD_ROW);
    let aad = air.aad.preprocessed_trace().expect("AAD");
    copy(values, shape, AAD_ROW, 1, aad);
    flags(values, shape, HKDF_ROW, 2, 512);
    let gcm = GcmVerticalAir.frame_preprocessed_trace();
    copy(values, shape, GCM_ROW, 3, gcm);
    flags(values, shape, NOTE_ROW, 4, 512);
    Some(())
}
