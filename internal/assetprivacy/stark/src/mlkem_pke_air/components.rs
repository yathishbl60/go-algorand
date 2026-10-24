// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemPkeAir, frames};
use crate::{
    Value, gcm_vertical::frame_builder::FrameBuilder, mlkem_ciphertext_air::MlKemCiphertextAir,
    mlkem_dot_air::MlKemDotAir, mlkem_ntt_air::MlKemNttAir,
};
use p3_air::{Air, AirBuilder};

impl MlKemPkeAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        MlKemNttAir::forward().eval(&mut FrameBuilder::new(builder, frames::ntt(false)));
        MlKemNttAir::inverse().eval(&mut FrameBuilder::new(builder, frames::ntt(true)));
        MlKemDotAir.eval(&mut FrameBuilder::new(builder, frames::dot()));
        MlKemCiphertextAir.eval(&mut FrameBuilder::new(builder, frames::codec()));
    }
}
