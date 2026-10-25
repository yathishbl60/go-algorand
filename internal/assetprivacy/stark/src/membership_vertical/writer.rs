// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{FRAME_ROWS, MembershipVerticalAir, STAGE_ROWS, WIDTH, witness::Prepared};
use crate::{Value, membership_air::MembershipPath};

impl MembershipVerticalAir {
    pub(crate) fn write_into(
        &self,
        leaf: [u8; 64],
        path: &MembershipPath,
        values: &mut [Value],
        target: (usize, usize, usize),
    ) -> Option<[u8; 64]> {
        check_target(values.len(), target)?;
        let input = self.prepare(leaf, path);
        self.write_rows(leaf, &input, values, target, FRAME_ROWS)?;
        Some(input.root)
    }

    pub(super) fn write_rows(
        &self,
        leaf: [u8; 64],
        input: &Prepared,
        values: &mut [Value],
        target: (usize, usize, usize),
        rows: usize,
    ) -> Option<()> {
        super::write::fixed(leaf, input, values, target, rows);
        for (level, stage) in input.stages.iter().enumerate() {
            let frame = (target.0, target.1, target.2 + level * STAGE_ROWS);
            self.hash
                .write_frame(&stage.message, values, frame, STAGE_ROWS)?;
        }
        Some(())
    }
}

fn check_target(len: usize, target: (usize, usize, usize)) -> Option<()> {
    let (stride, column, first) = target;
    if stride == 0 || !len.is_multiple_of(stride) || column.checked_add(WIDTH)? > stride {
        return None;
    }
    (first.checked_add(FRAME_ROWS)? <= len / stride).then_some(())
}
