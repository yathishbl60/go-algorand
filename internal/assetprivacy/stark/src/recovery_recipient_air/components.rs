// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{RecoveryRecipientAir, frames};
use crate::{Value, gcm_vertical::frame_builder::FrameBuilder};
use p3_air::{Air, AirBuilder, BaseAir};

impl RecoveryRecipientAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let air = &self.envelope;
        let frame = frames::component(0, air.width(), air.preprocessed_width(), 0..1068);
        air.eval(&mut FrameBuilder::new(builder, frame));
        for (index, air) in [&self.view, &self.spend].into_iter().enumerate() {
            let frame =
                frames::component(index + 1, air.width(), air.preprocessed_width(), 1068..1100);
            air.eval(&mut FrameBuilder::new(builder, frame));
        }
    }
}
