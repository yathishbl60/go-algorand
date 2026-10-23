// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_air::{AirBuilder, WindowAccess};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::NUM_KECCAK_COLS;

use super::NullifiedNoteAir;
use crate::Value;

impl NullifiedNoteAir {
    pub(super) fn eval_prefix<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let offset = self.owned.width() + NUM_KECCAK_COLS;
        for (i, byte) in self.prefix.iter().enumerate() {
            for bit in 0..8 {
                let expected = Value::from_bool(byte & (1 << bit) != 0);
                builder.assert_eq(main.current_slice()[offset + i * 8 + bit], expected);
            }
        }
    }

    pub(super) fn eval_serial<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let offset = self.owned.width() + NUM_KECCAK_COLS + self.prefix.len() * 8;
        for bit in 0..256 {
            let serial = main.current_slice()[self.owned.serial_bit_column(bit)];
            builder.assert_eq(main.current_slice()[offset + bit], serial);
        }
    }

    pub(super) fn eval_digest<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        for limb in 0..32 {
            let public = builder.public_values()[limb];
            builder.assert_eq(main.current_slice()[self.digest_offset() + limb], public);
        }
    }
}
