// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{PREPROCESSED, ROWS, schedule_row};
use crate::{
    Value, mlkem_ciphertext_air::MlKemCiphertextAir, mlkem_dot_air::MlKemDotAir,
    mlkem_ntt_air::MlKemNttAir,
};
use p3_air::BaseAir;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub(super) struct Tables {
    pub forward: Vec<Value>,
    pub inverse: Vec<Value>,
    pub dot: Vec<Value>,
    pub codec: Vec<Value>,
}

pub(super) fn build() -> RowMajorMatrix<Value> {
    let tables = tables();
    let mut values = Value::zero_vec(ROWS * PREPROCESSED);
    for (index, row) in values
        .as_chunks_mut::<PREPROCESSED>()
        .0
        .iter_mut()
        .enumerate()
    {
        schedule_row::write(row, index, &tables);
    }
    RowMajorMatrix::new(values, PREPROCESSED)
}

fn tables() -> Tables {
    let forward = ntt(false);
    let inverse = ntt(true);
    let dot = MlKemDotAir.preprocessed_trace().expect("dot").values;
    let codec = MlKemCiphertextAir
        .preprocessed_trace()
        .expect("codec")
        .values;
    Tables {
        forward,
        inverse,
        dot,
        codec,
    }
}

fn ntt(inverse: bool) -> Vec<Value> {
    let air = if inverse {
        MlKemNttAir::inverse()
    } else {
        MlKemNttAir::forward()
    };
    air.preprocessed_trace().expect("fixed NTT schedule").values
}
