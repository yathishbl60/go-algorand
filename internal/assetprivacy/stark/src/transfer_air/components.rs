// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{OUTPUT_START, PUBLIC_INPUT_WIDTH, TransferAir, builder::SegmentBuilder};
use crate::Value;
use p3_air::{Air, AirBuilder};

impl TransferAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for index in 0..2 {
            self.eval_input(builder, index);
            self.eval_output(builder, index);
        }
    }

    fn eval_input<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, index: usize) {
        let window = builder.main();
        let start = index * self.input.width();
        let public = index * PUBLIC_INPUT_WIDTH;
        self.input.eval(&mut SegmentBuilder {
            inner: builder,
            window,
            columns: start..start + self.input.width(),
            public: public..public + PUBLIC_INPUT_WIDTH,
        });
    }

    fn eval_output<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, index: usize) {
        let window = builder.main();
        let start = 2 * self.input.width() + index * self.output.width();
        let public = OUTPUT_START + index * 32;
        self.output.eval(&mut SegmentBuilder {
            inner: builder,
            window,
            columns: start..start + self.output.width(),
            public: public..public + 32,
        });
    }
}
