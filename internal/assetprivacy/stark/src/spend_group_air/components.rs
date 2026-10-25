// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{SpendGroupAir, frames, mapped_builder::MappedBuilder};
use crate::{Value, gcm_vertical::frame_builder::FrameBuilder};
use p3_air::{Air, AirBuilder, BaseAir};

impl SpendGroupAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for index in 0..2 {
            self.eval_input(builder, index);
        }
        for index in 0..self.operation.output_count() {
            let public = 128 + 32 * index;
            let frame = frames::component(index + 2, self.output.width(), 0, public..public + 32);
            self.output.eval(&mut FrameBuilder::new(builder, frame));
        }
    }
    fn eval_input<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, index: usize) {
        let frame = frames::component(
            index,
            self.input.width(),
            self.input.preprocessed_width(),
            0..96,
        );
        let eligibility = self.operation.eligibility_start();
        let public = [index * 64..index * 64 + 64, eligibility..eligibility + 32];
        self.input
            .eval(&mut MappedBuilder::new(builder, frame, &public));
    }
}
