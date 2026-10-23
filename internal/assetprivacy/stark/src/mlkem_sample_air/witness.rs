// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MlKemSampleAir;
use crate::Value;
use p3_field::PrimeCharacteristicRing;

pub(super) struct Step {
    pub candidate: u16,
    pub cursor: usize,
    pub active: bool,
}

impl MlKemSampleAir {
    pub(super) fn write_step(&self, row: &mut [Value], step: Step) -> bool {
        row[self.cursor_start() + step.cursor] = Value::ONE;
        write_bits(row, self.candidate_start(), step.candidate);
        write_bits(row, self.low_start(), (step.candidate + 767) & 4095);
        row[self.reject_column()] = Value::from_bool(step.candidate >= 3329);
        let accepted = step.active && step.candidate < 3329 && step.cursor < 256;
        row[self.accept_column()] = Value::from_bool(accepted);
        accepted
    }
}

fn write_bits(row: &mut [Value], start: usize, value: u16) {
    for bit in 0..12 {
        row[start + bit] = Value::from_bool(value & (1 << bit) != 0);
    }
}
