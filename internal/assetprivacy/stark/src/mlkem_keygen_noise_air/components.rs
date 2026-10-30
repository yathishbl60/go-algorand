// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemKeygenNoiseAir, NOISE_WIDTH, SEED_WIDTH, WORK};
use crate::{
    Value,
    gcm_vertical::{frame::Frame, frame_builder::FrameBuilder},
};
use p3_air::{Air, AirBuilder};

impl MlKemKeygenNoiseAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        self.seed
            .eval(&mut FrameBuilder::new(builder, frame(0, SEED_WIDTH)));
        for i in 0..6 {
            self.noise[i].eval(&mut FrameBuilder::new(builder, frame(i + 1, NOISE_WIDTH)));
        }
    }
}

fn frame(stage: usize, width: usize) -> Frame {
    Frame {
        columns: WORK..WORK + width,
        preprocessing: 0..0,
        public: 0..0,
        flags: 4 * stage,
    }
}
