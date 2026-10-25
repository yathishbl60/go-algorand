// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DIGEST, FLAGS, HASH, MembershipVerticalAir, WIDTH};
use crate::{
    Value,
    gcm_vertical::{frame::Frame, frame_builder::FrameBuilder},
    private_output_builder::PrivateOutputBuilder,
};
use p3_air::{Air, AirBuilder};

impl MembershipVerticalAir {
    pub(super) fn eval_hash<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let window = builder.main();
        let mut private = PrivateOutputBuilder {
            inner: builder,
            window,
            columns: 0..WIDTH,
            output: DIGEST..DIGEST + 32,
        };
        let frame = Frame {
            columns: 0..HASH,
            preprocessing: 0..0,
            public: 0..32,
            flags: FLAGS,
        };
        self.hash.eval(&mut FrameBuilder::new(&mut private, frame));
    }
}
