// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{GroupInput, PublicStatement, RecoveryGroupAir};

impl RecoveryGroupAir {
    pub(super) fn check_inputs(
        &self,
        input: &GroupInput<'_>,
        statement: &PublicStatement,
    ) -> Option<()> {
        statement.validate(self.operation)?;
        if input.outputs.len() != self.operation.output_count() {
            return None;
        }
        if self.input.check_inputs(input.input, statement.input.root)? != statement.input.nullifier
        {
            return None;
        }
        self.check_outputs(input, statement)
    }
    fn check_outputs(&self, input: &GroupInput<'_>, statement: &PublicStatement) -> Option<()> {
        for (opening, commitment) in input.outputs.iter().zip(&statement.outputs) {
            if self.output.commitment(opening)? != *commitment {
                return None;
            }
        }
        if let Some(index) = self.operation.remainder_index()
            && input.outputs[index][73..169] != input.input.opening[73..169]
        {
            return None;
        }
        Some(())
    }
}
