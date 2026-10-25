// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    CARRY, GroupInput, INPUT_ROWS, OUTPUT_ROW, OUTPUT_ROWS, PublicStatement, SpendGroupAir, WIDTH,
    WORK,
};
use crate::{Value, amounts::Amounts};

impl SpendGroupAir {
    pub(super) fn write_components(
        &self,
        input: &GroupInput<'_>,
        statement: &PublicStatement,
        values: &mut [Value],
    ) -> Option<()> {
        self.write_inputs(input, statement, values)?;
        for (index, opening) in input.outputs.iter().enumerate() {
            self.output.write_into(
                opening,
                values,
                (WIDTH, WORK, OUTPUT_ROW + index * OUTPUT_ROWS),
            )?;
        }
        Some(())
    }
    fn write_inputs(
        &self,
        input: &GroupInput<'_>,
        statement: &PublicStatement,
        values: &mut [Value],
    ) -> Option<()> {
        for index in 0..2 {
            let shape = (WIDTH, WORK, index * INPUT_ROWS);
            self.input.write_into(
                input.inputs[index],
                statement.inputs[index].root,
                statement.current_eligibility_root,
                values,
                shape,
            )?;
        }
        Some(())
    }
}

pub(super) fn write_amounts(values: &mut [Value], amounts: &Amounts<Value>) {
    for row in values.as_chunks_mut::<WIDTH>().0 {
        for (index, bits) in amounts.values.iter().enumerate() {
            row[index * 64..index * 64 + 64].copy_from_slice(bits);
        }
        row[CARRY..WORK].copy_from_slice(&amounts.carry);
    }
}
