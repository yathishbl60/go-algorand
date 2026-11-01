// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CompactKeccakAir, PREPROCESSED, ProgramColumn as P, STATE_BITS, WIDTH, values};
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeField64;

impl<F: PrimeField64> CompactKeccakAir<F> {
    pub(super) fn eval_bits<AB: AirBuilder<F = F>>(&self, builder: &mut AB) {
        let main = builder.main();
        let dummy = builder.preprocessed().current_slice()[P::Dummy as usize];
        for &bit in &main.current_slice()[..WIDTH] {
            builder.assert_bool(bit);
            builder.when(dummy).assert_zero(bit);
        }
    }

    pub(super) fn eval_round<AB: AirBuilder<F = F>>(&self, builder: &mut AB) {
        let main = builder.main();
        let (row, next) = (main.current_slice(), main.next_slice());
        let program: [AB::Var; PREPROCESSED] =
            core::array::from_fn(|i| builder.preprocessed().current_slice()[i]);
        let active = program[P::Continuation as usize];
        for index in 0..STATE_BITS {
            let input = values::input_bit::<AB::Expr, _>(next, index);
            let output = values::output_bit::<AB::Expr, _>(row, &program, index);
            builder
                .when_transition()
                .when(active)
                .assert_eq(input, output);
        }
    }
}
