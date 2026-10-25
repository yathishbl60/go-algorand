// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::NullifiedNoteAir;
use crate::{Value, nullifier_v2};
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::NUM_KECCAK_COLS;

impl NullifiedNoteAir {
    pub(super) fn eval_commitment_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let limbs = self
            .owned
            .commitment_limbs(&row[..self.owned.width()])
            .expect("owned note");
        let start = self.owned.width() + NUM_KECCAK_COLS + nullifier_v2::COMMITMENT_BYTE * 8;
        nullifier_v2::bind_commitment(builder, AB::Expr::ONE, limbs, &row[start..start + 512]);
    }
}
