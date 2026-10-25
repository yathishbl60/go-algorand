// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{GroupInput, PublicStatement, RecoveryGroupAir, input::amount};
use crate::{Value, amounts::Amounts};
use p3_field::{Field, PrimeCharacteristicRing};

pub(super) struct Prepared {
    pub amounts: Amounts<Value>,
    pub account: [u8; 32],
    pub owner: [u8; 64],
    pub positive: [Value; 2],
}

impl RecoveryGroupAir {
    pub(super) fn prepare(
        &self,
        input: &GroupInput<'_>,
        statement: &PublicStatement,
    ) -> Option<Prepared> {
        self.check_inputs(input, statement)?;
        let outgoing = self.outgoing(input);
        let positive = self.positivity(outgoing)?;
        let incoming = [amount(input.input.opening), 0];
        let amounts = Amounts::witness(incoming, outgoing, self.operation.conversion())?;
        Some(Prepared {
            amounts,
            account: input.input.opening[137..169].try_into().ok()?,
            owner: input.input.opening[73..137].try_into().ok()?,
            positive,
        })
    }
    fn outgoing(&self, input: &GroupInput<'_>) -> [u64; 2] {
        let mut outgoing = [0; 2];
        for (index, opening) in input.outputs.iter().enumerate() {
            outgoing[self.operation.amount_slot(index) - 2] = amount(opening);
        }
        outgoing
    }
    fn positivity(&self, outgoing: [u64; 2]) -> Option<[Value; 2]> {
        let mut inverse = [Value::ZERO; 2];
        for (index, required) in [self.operation.private(), self.operation.partial()]
            .into_iter()
            .enumerate()
        {
            if required {
                if outgoing[index] == 0 {
                    return None;
                }
                inverse[index] = Value::from_u32(outgoing[index].count_ones()).inverse();
            }
        }
        Some(inverse)
    }
}
