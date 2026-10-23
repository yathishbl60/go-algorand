// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::HmacAir;
use crate::{Value, private_output_builder::PrivateOutputBuilder};
use p3_air::{Air, AirBuilder};

impl HmacAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let mut start = 0;
        for (index, component) in [&self.inner, &self.outer].into_iter().enumerate() {
            let window = builder.main();
            let output = self.digest_start() + index * 32;
            component.eval(&mut PrivateOutputBuilder {
                inner: builder,
                window,
                columns: start..start + component.width(),
                output: output..output + 32,
            });
            start += component.width();
        }
    }
}
