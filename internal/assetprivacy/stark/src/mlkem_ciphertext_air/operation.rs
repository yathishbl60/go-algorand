// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemCiphertextAir, view};
use crate::{Value, mlkem_field::eval_compression};
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl MlKemCiphertextAir {
    pub(super) fn eval_operation<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let ten: AB::Expr = builder.preprocessed().current_slice()[..768]
            .iter()
            .copied()
            .map(Into::into)
            .sum();
        eval_compression(
            &mut builder.when(ten.clone()),
            &view::compression::<_, 10>(row),
        );
        eval_compression(
            &mut builder.when(AB::Expr::ONE - ten),
            &view::compression::<_, 4>(row),
        );
    }
}
