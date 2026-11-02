// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{OwnedNotesAir, schema::*};
use p3_air::{AirBuilder, WindowAccess};
use p3_field::{PrimeCharacteristicRing, PrimeField64};

impl<F: PrimeField64> OwnedNotesAir<F> {
    pub(super) fn eval_private_input<AB: AirBuilder<F = F>>(&self, builder: &mut AB) {
        self.eval_known_input(builder);
        let main = builder.main();
        let head: AB::Expr = builder.periodic_values()[HASH_HEADER].into();
        let inverse = main.current_slice()[INVERSE];
        let count = self.secret_count::<AB>(&main.current_slice()[HASH..HASH + 1920]);
        builder.when(head.clone()).assert_one(count * inverse);
        builder.when(AB::Expr::ONE - head).assert_zero(inverse);
    }

    fn eval_known_input<AB: AirBuilder<F = F>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = &main.current_slice()[HASH..HASH + 1920];
        let head = builder.periodic_values()[HASH_HEADER];
        for index in (0..1600).filter(|index| !(40..296).contains(index)) {
            let value = self
                .hash
                .input_bit::<AB::Expr, _>(row, index)
                .expect("fixed state width");
            builder.when(head).assert_eq(value, self.known_bit(index));
        }
    }

    fn secret_count<AB: AirBuilder<F = F>>(&self, row: &[AB::Var]) -> AB::Expr {
        (40..296)
            .map(|index| {
                self.hash
                    .input_bit::<AB::Expr, _>(row, index)
                    .expect("fixed state width")
                    .xor(&self.known_bit(index).into())
            })
            .sum()
    }

    fn known_bit(&self, index: usize) -> F {
        F::from_bool(self.input_base[index / 64] & (1u64 << (index % 64)) != 0)
    }
}
