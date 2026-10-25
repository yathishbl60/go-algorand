// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CurrentEligibilityAir, frames};
use crate::{Value, gcm_vertical::frame_builder::FrameBuilder};
use p3_air::{Air, AirBuilder, BaseAir};

impl CurrentEligibilityAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let air = &self.record;
        let frame = frames::component(0, air.width(), 0, 0);
        air.eval(&mut FrameBuilder::new(builder, frame));
        let air = &self.membership;
        let frame = frames::component(1, air.width(), air.preprocessed_width(), 32);
        air.eval(&mut FrameBuilder::new(builder, frame));
    }
}
