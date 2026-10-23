// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MembershipAir, trace::Stage};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::{Matrix, dense::RowMajorMatrix};

impl MembershipAir {
    pub(super) fn join(&self, stages: Vec<Stage>, leaf: [Value; 32]) -> RowMajorMatrix<Value> {
        let height = stages[0].hash.height();
        let mut values = Value::zero_vec(height * self.width());
        for (r, row) in values.chunks_exact_mut(self.width()).enumerate() {
            for (level, stage) in stages.iter().enumerate() {
                self.write_stage(row, r, level, stage);
            }
            row[self.leaf_start()..].copy_from_slice(&leaf);
        }
        RowMajorMatrix::new(values, self.width())
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
