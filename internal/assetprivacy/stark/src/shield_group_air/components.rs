// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{NOTE_WIDTH, ShieldGroupAir, WORK};
use crate::{
    Value,
    gcm_vertical::{frame::Frame, frame_builder::FrameBuilder},
};
use p3_air::{Air, AirBuilder, WindowAccess};

impl ShieldGroupAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for index in 0..2 {
            let frame = Frame {
                columns: WORK..WORK + NOTE_WIDTH,
                preprocessing: 0..0,
                public: index * 32..index * 32 + 32,
                flags: index * 4,
            };
            self.output.eval(&mut FrameBuilder::new(builder, frame));
        }
    }
    pub(super) fn eval_output_amounts<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for index in 0..2 {
            let main = builder.main();
            let row = main.current_slice();
            let value = self
                .output
                .value_bits(&row[WORK..WORK + NOTE_WIDTH])
                .expect("note width");
            let flag = builder.preprocessed().current_slice()[index * 4];
            for (&root, bit) in row[(index + 2) * 64..(index + 3) * 64].iter().zip(value) {
                builder.when(flag).assert_eq(root, bit);
            }
        }
    }
}
