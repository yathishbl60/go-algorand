// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AAD, CHILD_PRE, EnvelopeAir, KEY, SHARED, WORK};
use crate::Value;
use p3_air::{AirBuilder, WindowAccess};

impl EnvelopeAir {
    pub(super) fn eval_hkdf_binding<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let child = &row[WORK..WORK + self.hkdf.width()];
        let flag = builder.preprocessed().current_slice()[CHILD_PRE + 8];
        for bit in 0..256 {
            let column = self
                .hkdf
                .shared_bit_column(bit / 8, bit % 8)
                .expect("shared");
            builder
                .when(flag)
                .assert_eq(row[SHARED + bit], child[column]);
        }
        self.bind_salt_key(builder);
    }

    fn bind_salt_key<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let child = &row[WORK..WORK + self.hkdf.width()];
        let flag = builder.preprocessed().current_slice()[CHILD_PRE + 8];
        let salt = self.hkdf.salt_bits(child).expect("salt");
        let key = self.hkdf.output_bits(child).expect("AES key");
        for (index, &bit) in salt.iter().flatten().enumerate() {
            builder.when(flag).assert_eq(row[AAD + index], bit);
        }
        for (index, &bit) in key.iter().flatten().enumerate() {
            builder.when(flag).assert_eq(row[KEY + index], bit);
        }
    }
}
