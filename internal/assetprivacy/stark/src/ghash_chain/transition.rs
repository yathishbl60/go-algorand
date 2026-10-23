// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{BITS, DATA, GhashChainAir, SELECTED, V, X, Y, Z, schedule::active};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl GhashChainAir {
    pub(super) fn eval_transitions<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let end: AB::Expr = builder.preprocessed().current_slice()[BITS - 1].into();
        let active = active::<AB>(builder);
        let gates = [active.clone() - end.clone(), AB::Expr::ONE - active];
        for bit in 0..BITS {
            self.eval_product_step(builder, bit, gates.clone());
            self.eval_chain_step(builder, bit, end.clone());
        }
    }

    fn eval_product_step<AB: AirBuilder<F = Value>>(
        &self,
        builder: &mut AB,
        bit: usize,
        gates: [AB::Expr; 2],
    ) {
        let main = builder.main();
        let (row, next) = (main.current_slice(), main.next_slice());
        let sum = AB::Expr::from(row[Z + bit]).xor(&row[SELECTED + bit].into());
        {
            let mut transitions = builder.when_transition();
            let mut running = transitions.when(gates[0].clone());
            running.assert_eq(next[Z + bit], sum);
            running.assert_eq(next[V + bit], shift::<AB>(row, bit));
        }
        hold(builder, bit, gates[1].clone(), [V, Z]);
    }

    fn eval_chain_step<AB: AirBuilder<F = Value>>(
        &self,
        builder: &mut AB,
        bit: usize,
        end: AB::Expr,
    ) {
        let main = builder.main();
        let (row, next) = (main.current_slice(), main.next_slice());
        let product = AB::Expr::from(row[Z + bit]).xor(&row[SELECTED + bit].into());
        let prior: AB::Expr = row[Y + bit].into();
        let next_y = prior.clone() + end.clone() * (product - prior);
        builder.when_transition().assert_eq(next[Y + bit], next_y);
        hold(builder, bit, AB::Expr::ONE - end, [X, DATA]);
    }
}

fn hold<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    bit: usize,
    gate: AB::Expr,
    offsets: [usize; 2],
) {
    let main = builder.main();
    for offset in offsets {
        builder.when_transition().when(gate.clone()).assert_eq(
            main.next_slice()[offset + bit],
            main.current_slice()[offset + bit],
        );
    }
}

fn shift<AB: AirBuilder<F = Value>>(row: &[AB::Var], bit: usize) -> AB::Expr {
    let prior = if bit == 0 {
        AB::Expr::ZERO
    } else {
        row[V + bit - 1].into()
    };
    if matches!(bit, 0 | 1 | 2 | 7) {
        prior.xor(&row[V + BITS - 1].into())
    } else {
        prior
    }
}
