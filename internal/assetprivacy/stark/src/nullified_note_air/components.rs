// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_air::{Air, AirBuilder};

use super::NullifiedNoteAir;
use crate::{Value, prefix_builder::PrefixBuilder, private_output_builder::PrivateOutputBuilder};

impl NullifiedNoteAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        self.owned.eval(&mut PrefixBuilder {
            inner: builder,
            width: self.owned.width(),
        });
        let window = builder.main();
        let columns = self.owned.width()..self.digest_offset();
        let output = self.digest_offset()..self.width();
        self.hash.eval(&mut PrivateOutputBuilder {
            inner: builder,
            window,
            columns,
            output,
        });
    }
}
