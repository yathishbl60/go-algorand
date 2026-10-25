// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{COMMIT, OWNER, SpendableNoteAir, WIDTH, frames};
use crate::{
    Value, gcm_vertical::frame_builder::FrameBuilder, private_output_builder::PrivateOutputBuilder,
};
use p3_air::{Air, AirBuilder, BaseAir};

impl SpendableNoteAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        self.eval_note(builder);
        self.eval_owner(builder);
        let air = &self.nullifier;
        let frame = frames::component(2, air.width(), 0, 0..32);
        air.eval(&mut FrameBuilder::new(builder, frame));
        let air = &self.membership;
        let frame = frames::component(3, air.width(), air.preprocessed_width(), 32..64);
        air.eval(&mut FrameBuilder::new(builder, frame));
        let air = &self.eligibility;
        let frame = frames::component(4, air.width(), air.preprocessed_width(), 64..96);
        air.eval(&mut FrameBuilder::new(builder, frame));
    }
    fn eval_note<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let window = builder.main();
        let mut private = PrivateOutputBuilder {
            inner: builder,
            window,
            columns: 0..WIDTH,
            output: COMMIT..COMMIT + 32,
        };
        let frame = frames::component(0, self.note.width(), 0, 0..32);
        self.note.eval(&mut FrameBuilder::new(&mut private, frame));
    }
    fn eval_owner<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let window = builder.main();
        let mut private = PrivateOutputBuilder {
            inner: builder,
            window,
            columns: 0..WIDTH,
            output: OWNER..OWNER + 32,
        };
        let frame = frames::component(1, self.owner.width(), 0, 0..32);
        self.owner.eval(&mut FrameBuilder::new(&mut private, frame));
    }
}
