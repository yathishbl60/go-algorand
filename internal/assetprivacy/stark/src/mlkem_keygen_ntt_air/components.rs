// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{FLAGS, MlKemKeygenNttAir, NOISE_SCHEDULE, NOISE_WIDTH, NTT_SCHEDULE, NTT_WIDTH, WORK};
use crate::{
    Value,
    gcm_vertical::{frame::Frame, frame_builder::FrameBuilder},
};
use p3_air::{Air, AirBuilder};

impl MlKemKeygenNttAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        self.noise.eval(&mut FrameBuilder::new(
            builder,
            Frame {
                columns: 0..NOISE_WIDTH,
                preprocessing: 0..NOISE_SCHEDULE,
                public: 0..0,
                flags: FLAGS,
            },
        ));
        for i in 0..6 {
            self.ntt.eval(&mut FrameBuilder::new(builder, ntt_frame(i)));
        }
    }
}

fn ntt_frame(i: usize) -> Frame {
    Frame {
        columns: WORK..WORK + NTT_WIDTH,
        preprocessing: NOISE_SCHEDULE..NOISE_SCHEDULE + NTT_SCHEDULE,
        public: 0..0,
        flags: FLAGS + 4 * (i + 1),
    }
}
