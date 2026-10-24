// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AES_FLAGS, AES_SCHEDULE, GH_FLAGS, GH_ROW, GH_SCHEDULE, PREPROCESSED, ROWS};
use crate::{Value, aes_gcm_vertical::AesGcmAir, ghash_chain::GhashChainAir};
use p3_air::BaseAir;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub(super) fn schedule() -> RowMajorMatrix<Value> {
    let aes = AesGcmAir.preprocessed_trace().expect("fixed AES schedule");
    let hash = GhashChainAir
        .preprocessed_trace()
        .expect("fixed GHASH schedule");
    let mut values = Value::zero_vec(ROWS * PREPROCESSED);
    for (index, row) in values
        .as_chunks_mut::<PREPROCESSED>()
        .0
        .iter_mut()
        .enumerate()
    {
        write_schedule(row, index, &aes.values, &hash.values);
    }
    RowMajorMatrix::new(values, PREPROCESSED)
}

fn write_schedule(row: &mut [Value], index: usize, aes: &[Value], hash: &[Value]) {
    if index < GH_ROW {
        row[..AES_SCHEDULE].copy_from_slice(&aes[index * AES_SCHEDULE..][..AES_SCHEDULE]);
        frame(row, AES_FLAGS, index, GH_ROW);
    } else {
        let start = (index - GH_ROW) * GH_SCHEDULE;
        row[AES_SCHEDULE..AES_FLAGS].copy_from_slice(&hash[start..start + GH_SCHEDULE]);
        frame(row, GH_FLAGS, index - GH_ROW, ROWS - GH_ROW);
    }
}

fn frame(row: &mut [Value], start: usize, index: usize, rows: usize) {
    row[start] = Value::ONE;
    row[start + 1] = Value::from_bool(index == 0);
    row[start + 2] = Value::from_bool(index == rows - 1);
    row[start + 3] = Value::from_bool(index < rows - 1);
}
