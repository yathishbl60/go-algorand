// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    BIT_COUNT, BYTE_BITS, END_INVERSE, NoteBytesAir, PREVIOUS, ProgramColumn as P, WIDTH, values,
};
use p3_air::{AirBuilder, WindowAccess};
use p3_field::{PrimeCharacteristicRing, PrimeField64};

impl<F: PrimeField64> NoteBytesAir<F> {
    pub(super) fn eval_bytes<AB: AirBuilder<F = F>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for &bit in &row[..BYTE_BITS] {
            builder.assert_bool(bit);
        }
        let dummy = builder.preprocessed().current_slice()[P::Dummy as usize];
        for &value in &row[..WIDTH] {
            builder.when(dummy).assert_zero(value);
        }
    }

    pub(super) fn eval_public<AB: AirBuilder<F = F>>(&self, builder: &mut AB) {
        let main = builder.main();
        let prep = builder.preprocessed();
        let known = prep.current_slice()[P::ExpectedByte as usize];
        let active = prep.current_slice()[P::PublicByte as usize];
        let byte = values::byte::<AB::Expr, _>(&main.current_slice()[..BYTE_BITS]);
        builder.when(active).assert_eq(byte, known);
    }

    pub(super) fn eval_continuity<AB: AirBuilder<F = F>>(&self, builder: &mut AB) {
        let main = builder.main();
        let (row, next) = (main.current_slice(), main.next_slice());
        let prep = builder.preprocessed();
        let active = prep.current_slice()[P::NonzeroActive as usize];
        let end = prep.current_slice()[P::NonzeroEnd as usize];
        let byte = values::byte::<AB::Expr, _>(&row[..BYTE_BITS]);
        let count = values::count::<AB::Expr, _>(&row[..BYTE_BITS]);
        let sum = (AB::Expr::ONE - end) * (row[BIT_COUNT].into() + active * count);
        builder.when_transition().assert_eq(next[PREVIOUS], byte);
        builder.when_transition().assert_eq(next[BIT_COUNT], sum);
        builder.when_first_row().assert_zero(row[PREVIOUS]);
        builder.when_first_row().assert_zero(row[BIT_COUNT]);
    }

    pub(super) fn eval_nonzero<AB: AirBuilder<F = F>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let end = builder.preprocessed().current_slice()[P::NonzeroEnd as usize];
        let count = values::count::<AB::Expr, _>(&row[..BYTE_BITS]);
        builder
            .when(end)
            .assert_one((row[BIT_COUNT].into() + count) * row[END_INVERSE]);
        builder
            .when(AB::Expr::ONE - end)
            .assert_zero(row[END_INVERSE]);
    }
}
