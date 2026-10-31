// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{GroupInput, PublicStatement, SpendGroupAir, input::amount};
use crate::{Value, amounts::Amounts};

impl SpendGroupAir {
    pub(super) fn prepare(
        &self,
        input: &GroupInput<'_>,
        statement: &PublicStatement,
    ) -> Option<Amounts<Value>> {
        statement.validate(self.operation)?;
        if input.outputs.len() != self.operation.output_count() {
            return None;
        }
        self.check_participant_openings(input)?;
        self.check_inputs(input, statement)?;
        self.check_outputs(input, statement)?;
        let incoming = input.inputs.map(|input| amount(input.opening));
        let mut outgoing = [0; 2];
        for (index, opening) in input.outputs.iter().enumerate() {
            outgoing[index] = amount(opening);
        }
        Amounts::witness(incoming, outgoing, self.operation.conversion())
    }
    fn check_inputs(&self, input: &GroupInput<'_>, statement: &PublicStatement) -> Option<()> {
        for index in 0..2 {
            let expected = &statement.inputs[index];
            let nullifier = self.input.check_inputs(
                input.inputs[index],
                expected.root,
                statement.current_eligibility_root,
            )?;
            if nullifier != expected.nullifier {
                return None;
            }
        }
        Some(())
    }
    fn check_outputs(&self, input: &GroupInput<'_>, statement: &PublicStatement) -> Option<()> {
        for (opening, commitment) in input.outputs.iter().zip(&statement.outputs) {
            if self.output.commitment(opening)? != *commitment {
                return None;
            }
        }
        Some(())
    }
}
