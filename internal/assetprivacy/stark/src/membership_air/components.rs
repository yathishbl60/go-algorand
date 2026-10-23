// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DEPTH, MembershipAir};
use crate::{Value, private_output_builder::PrivateOutputBuilder};
use p3_air::{Air, AirBuilder};

impl MembershipAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for level in 0..DEPTH {
            let window = builder.main();
            let start = self.stage_start(level);
            let digest = self.digest_start(level);
            self.hash.eval(&mut PrivateOutputBuilder {
                inner: builder,
                window,
                columns: start..digest,
                output: digest..digest + 32,
            });
        }
    }
}
