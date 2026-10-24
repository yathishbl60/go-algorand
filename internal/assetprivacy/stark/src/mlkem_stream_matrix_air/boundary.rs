// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{BUFFER, LATER, MlKemStreamMatrixAir, SEED, binding::bits};
use crate::Value;
use core::borrow::Borrow;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::KeccakCols;

impl MlKemStreamMatrixAir {
    pub(super) fn eval_initial<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let keccak: &KeccakCols<AB::Var> = row[..SEED].borrow();
        let later = builder.preprocessed().current_slice()[LATER];
        for limb in 0..100 {
            let actual = keccak.preimage[limb / 20][limb / 4 % 5][limb % 4];
            builder
                .when_first_row()
                .assert_eq(actual, self.initial_limb::<AB>(row, limb));
            builder.when(later).assert_eq(actual, row[BUFFER + limb]);
        }
    }
    fn initial_limb<AB: AirBuilder<F = Value>>(&self, row: &[AB::Var], limb: usize) -> AB::Expr {
        if limb < 16 {
            return bits::<AB>(&row[SEED + 16 * limb..SEED + 16 * (limb + 1)]);
        }
        let value = match limb {
            16 => u16::from(self.first) | (u16::from(self.second) << 8),
            17 => 0x1f,
            83 => 0x8000,
            _ => 0,
        };
        Value::from_u16(value).into()
    }
}
