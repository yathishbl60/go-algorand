// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_air::{Air, AirBuilder};

use super::OwnedNoteAir;
use crate::{Value, private_output_builder::PrivateOutputBuilder};

impl OwnedNoteAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        self.eval_note(builder);
        self.eval_owner(builder);
    }

    fn eval_note<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let window = builder.main();
        let output = self.digest_offset()..self.digest_offset() + 32;
        let mut child = PrivateOutputBuilder {
            inner: builder,
            window,
            columns: 0..self.note.width(),
            output,
        };
        self.note.eval(&mut child);
    }

    fn eval_owner<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let window = builder.main();
        let columns = self.note.width()..self.digest_offset();
        let output = self.digest_offset() + 32..self.digest_offset() + 64;
        let mut child = PrivateOutputBuilder {
            inner: builder,
            window,
            columns,
            output,
        };
        self.owner.eval(&mut child);
    }
}
