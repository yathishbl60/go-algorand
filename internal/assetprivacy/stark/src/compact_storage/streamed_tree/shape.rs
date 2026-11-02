// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MaskedTrace, SALT_ELEMS, TwoAdicMultiplicativeCoset, Value32};
use p3_field::{PrimeCharacteristicRing, TwoAdicField};

pub(super) struct Shape {
    pub(super) rows: usize,
    pub(super) widths: Vec<usize>,
}

impl Shape {
    pub(super) fn new(
        inputs: &[&MaskedTrace<Value32>],
        domain: TwoAdicMultiplicativeCoset<Value32>,
        tile: usize,
        budget: usize,
    ) -> Option<Self> {
        if !(1..=16).contains(&inputs.len()) || tile == 0 || domain.shift() == Value32::ZERO {
            return None;
        }
        let rows = 1usize.checked_shl(u32::try_from(domain.log_size()).ok()?)?;
        if !valid_inputs(inputs, domain.log_size(), rows, tile, budget) {
            return None;
        }
        let widths: Vec<_> = inputs.iter().map(|input| input.width()).collect();
        widths
            .iter()
            .try_fold(0usize, |n, w| n.checked_add(*w)?.checked_add(SALT_ELEMS))?;
        Some(Self { rows, widths })
    }
}

fn valid_inputs(
    inputs: &[&MaskedTrace<Value32>],
    log: usize,
    rows: usize,
    tile: usize,
    budget: usize,
) -> bool {
    log <= Value32::TWO_ADICITY
        && inputs.iter().all(|input| {
            let coefficient_log = input.coefficient_rows().ilog2() as usize;
            input.coefficient_rows() == inputs[0].coefficient_rows()
                && coefficient_log <= log
                && rows
                    .checked_mul(tile.min(input.width()))
                    .is_some_and(|n| n <= budget)
                && Value32::two_adic_generator(log).exp_power_of_2(log - coefficient_log)
                    == Value32::two_adic_generator(coefficient_log)
        })
}
