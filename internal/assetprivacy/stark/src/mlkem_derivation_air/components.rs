// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MlKemDerivationAir;
use crate::{Value, private_output_builder::PrivateOutputBuilder};
use p3_air::{Air, AirBuilder};

impl MlKemDerivationAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let mut columns = 0;
        let mut output = self.digest_start();
        for component in [&self.key_hash, &self.expand] {
            let window = builder.main();
            let length = component.digest_bytes() / 2;
            component.eval(&mut PrivateOutputBuilder {
                inner: builder,
                window,
                columns: columns..columns + component.width(),
                output: output..output + length,
            });
            columns += component.width();
            output += length;
        }
    }
}
