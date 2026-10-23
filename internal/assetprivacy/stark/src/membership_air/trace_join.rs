// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MembershipAir, trace::Stage};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::Matrix;

impl MembershipAir {
    pub(super) fn empty_trace(&self, height: usize, leaf: [Value; 32]) -> Vec<Value> {
        let mut values = Value::zero_vec(height * self.width());
        for row in values.chunks_exact_mut(self.width()) {
            row[self.leaf_start()..].copy_from_slice(&leaf);
        }
        values
    }

    pub(super) fn copy_stage(
        &self,
        values: &mut [Value],
        level: usize,
        stage: &Stage,
    ) -> Option<()> {
        if values.len() != stage.hash.height() * self.width() {
            return None;
        }
        for (r, row) in values.chunks_exact_mut(self.width()).enumerate() {
            self.write_stage(row, r, level, stage);
        }
        Some(())
    }

    fn write_stage(&self, row: &mut [Value], r: usize, level: usize, stage: &Stage) {
        let (start, digest, width) = (
            self.stage_start(level),
            self.digest_start(level),
            self.hash.width(),
        );
        row[start..digest].copy_from_slice(&stage.hash.values[r * width..(r + 1) * width]);
        row[digest..digest + 32].copy_from_slice(&stage.digest);
        row[self.direction_column(level)] = Value::from_bool(stage.direction);
    }
}
