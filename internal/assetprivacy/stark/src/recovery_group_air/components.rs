// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{RecoveryGroupAir, frames};
use crate::{Value, gcm_vertical::frame_builder::FrameBuilder};
use p3_air::{Air, AirBuilder, BaseAir};

impl RecoveryGroupAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let air = &self.input;
        let frame = frames::component(0, air.width(), air.preprocessed_width(), 0..64);
        air.eval(&mut FrameBuilder::new(builder, frame));
        for index in 0..self.operation.output_count() {
            let public = 64 + 32 * index;
            let frame = frames::component(index + 1, self.output.width(), 0, public..public + 32);
            self.output.eval(&mut FrameBuilder::new(builder, frame));
        }
    }
}
