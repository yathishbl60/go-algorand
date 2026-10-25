// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{GroupInput, INPUT_ROWS, OUTPUT_ROWS, PublicStatement, RecoveryGroupAir, WIDTH, WORK};
use crate::Value;

impl RecoveryGroupAir {
    pub(super) fn write_components(
        &self,
        input: &GroupInput<'_>,
        statement: &PublicStatement,
        values: &mut [Value],
    ) -> Option<()> {
        self.input
            .write_into(input.input, statement.input.root, values, (WIDTH, WORK, 0))?;
        for (index, opening) in input.outputs.iter().enumerate() {
            self.output.write_into(
                opening,
                values,
                (WIDTH, WORK, INPUT_ROWS + index * OUTPUT_ROWS),
            )?;
        }
        Some(())
    }
}
