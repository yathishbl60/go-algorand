// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemEncapsulationAir, frames};
use crate::{
    Value, gcm_vertical::frame_builder::FrameBuilder, mlkem_noise_air::MlKemNoiseAir,
    mlkem_pke_air::MlKemPkeAir, mlkem_stream_derivation_air::MlKemStreamDerivationAir,
    mlkem_stream_matrix_air::MlKemStreamMatrixAir,
};
use p3_air::{Air, AirBuilder, BaseAir};

impl MlKemEncapsulationAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let derivation = MlKemStreamDerivationAir;
        let frame = frames::component(0, derivation.width(), derivation.preprocessed_width());
        derivation.eval(&mut FrameBuilder::new(builder, frame));
        self.eval_matrices(builder);
        self.eval_noise(builder);
        MlKemPkeAir.eval(&mut FrameBuilder::new(builder, frames::pke()));
    }

    fn eval_matrices<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for index in 0..9 {
            let air = MlKemStreamMatrixAir::new((index % 3) as u8, (index / 3) as u8)
                .expect("fixed matrix coordinate");
            let frame = frames::component(1 + index, air.width(), air.preprocessed_width());
            air.eval(&mut FrameBuilder::new(builder, frame));
        }
    }

    fn eval_noise<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for index in 0..7 {
            let air = MlKemNoiseAir::new(index as u8).expect("fixed noise nonce");
            let frame = frames::component(10 + index, air.width(), air.preprocessed_width());
            air.eval(&mut FrameBuilder::new(builder, frame));
        }
    }
}
