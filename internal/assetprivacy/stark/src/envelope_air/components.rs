// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{COMMIT, EPOCH, EnvelopeAir, WIDTH, frames};
use crate::{
    Value,
    gcm_vertical::{GcmVerticalAir, frame_builder::FrameBuilder},
    mlkem_encapsulation_air::MlKemEncapsulationAir,
    private_output_builder::PrivateOutputBuilder,
};
use p3_air::{Air, AirBuilder, BaseAir};

impl EnvelopeAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let kem = MlKemEncapsulationAir;
        let frame = frames::component(0, kem.width(), kem.preprocessed_width(), 248..792);
        kem.eval(&mut FrameBuilder::new(builder, frame));
        let frame = frames::component(1, self.aad.width(), self.aad.preprocessed_width(), 248..792);
        self.aad.eval(&mut FrameBuilder::new(builder, frame));
        let frame = frames::component(2, self.hkdf.width(), 0, 0..0);
        self.hkdf.eval(&mut FrameBuilder::new(builder, frame));
        let frame = frames::component(3, GcmVerticalAir.width(), 234, 792..1068);
        GcmVerticalAir.eval(&mut FrameBuilder::new(builder, frame));
        self.eval_note_component(builder);
    }

    fn eval_note_component<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let window = builder.main();
        let mut private = PrivateOutputBuilder {
            inner: builder,
            window,
            columns: 0..WIDTH,
            output: COMMIT..EPOCH,
        };
        let frame = frames::component(4, self.note.width(), 0, 0..32);
        self.note.eval(&mut FrameBuilder::new(&mut private, frame));
    }
}
