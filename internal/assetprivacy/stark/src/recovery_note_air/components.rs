// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{COMMIT, RecoveryNoteAir, WIDTH, WORK, frames};
use crate::{
    Value, gcm_vertical::frame_builder::FrameBuilder, private_output_builder::PrivateOutputBuilder,
};
use p3_air::{Air, AirBuilder, BaseAir};

impl RecoveryNoteAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        self.eval_note(builder);
        let air = &self.nullifier;
        let frame = frames::component(1, air.width(), 0, 0..32);
        air.eval(&mut FrameBuilder::new(builder, frame));
        let air = &self.membership;
        let frame = frames::component(2, air.width(), air.preprocessed_width(), 32..64);
        air.eval(&mut FrameBuilder::new(builder, frame));
    }
    fn eval_note<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let window = builder.main();
        let mut private = PrivateOutputBuilder {
            inner: builder,
            window,
            columns: 0..WIDTH,
            output: COMMIT..WORK,
        };
        let frame = frames::component(0, self.note.width(), 0, 0..32);
        self.note.eval(&mut FrameBuilder::new(&mut private, frame));
    }
}
