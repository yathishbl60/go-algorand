// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CORE_PRE, CORE_WIDTH, DOT_PRE, DOT_WIDTH, FLAGS, MlKemKeygenAir};
use crate::{
    Value,
    gcm_vertical::{frame::Frame, frame_builder::FrameBuilder},
};
use p3_air::{Air, AirBuilder};

impl MlKemKeygenAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        self.core
            .eval(&mut FrameBuilder::new(builder, core_frame()));
        self.dot.eval(&mut FrameBuilder::new(builder, dot_frame()));
    }
}

fn core_frame() -> Frame {
    Frame {
        columns: 0..CORE_WIDTH,
        preprocessing: 0..CORE_PRE,
        public: 0..0,
        flags: FLAGS,
    }
}
fn dot_frame() -> Frame {
    Frame {
        columns: 0..DOT_WIDTH,
        preprocessing: CORE_PRE..CORE_PRE + DOT_PRE,
        public: 0..0,
        flags: FLAGS + 4,
    }
}
