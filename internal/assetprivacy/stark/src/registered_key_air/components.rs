// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{RegisteredKeyAir, frames, record::RecordAir};
use crate::{Value, gcm_vertical::frame_builder::FrameBuilder};
use p3_air::{Air, AirBuilder, BaseAir};

impl RegisteredKeyAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let frame = frames::component(0, self.record.width(), self.record.pre_width(), 0);
        let mut child = FrameBuilder::new(builder, frame);
        match &self.record {
            RecordAir::View(air) => air.eval(&mut child),
            RecordAir::Spend(air) => air.eval(&mut child),
        }
        let air = &self.membership;
        let frame = frames::component(1, air.width(), air.preprocessed_width(), 32);
        air.eval(&mut FrameBuilder::new(builder, frame));
    }
}
