// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{HkdfAir, INFO};
use crate::{Value, public_segment_builder::SegmentBuilder};
use p3_air::{Air, AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;

impl HkdfAir {
    pub(super) fn eval_components<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let mut start = 0;
        for component in [&self.extract, &self.expand] {
            let window = builder.main();
            component.eval(&mut SegmentBuilder {
                inner: builder,
                window,
                columns: start..start + component.width(),
                public: 0..0,
            });
            start += component.width();
        }
    }

    pub(super) fn eval_prk<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let window = builder.main();
        let row = window.current_slice();
        let start = self.extract.width();
        let output = self
            .extract
            .output_bits(&row[..start])
            .expect("extract width");
        let key = self
            .expand
            .key_bits(&row[start..self.width()])
            .expect("expand width");
        for byte in 0..64 {
            for bit in 0..8 {
                builder.assert_eq(output[byte][bit], key[byte][bit]);
            }
        }
    }

    pub(super) fn eval_info<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let window = builder.main();
        let row = window.current_slice();
        for (byte, expected) in INFO.iter().copied().chain(core::iter::once(1)).enumerate() {
            for bit in 0..8 {
                let column = self
                    .expand
                    .data_bit_column(byte, bit)
                    .expect("fixed info length");
                builder.assert_eq(
                    row[self.extract.width() + column],
                    Value::from_bool(expected & (1 << bit) != 0),
                );
            }
        }
    }
}
