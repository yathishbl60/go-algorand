// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Gcm;
use crate::{Value, aes_air::eval_byte};
use p3_air::AirBuilder;

pub(super) fn eval_ranges<AB: AirBuilder<F = Value>>(builder: &mut AB, value: &Gcm<AB::Var>) {
    for byte in value
        .nonce
        .iter()
        .chain(&value.associated)
        .chain(&value.plaintext)
        .chain(&value.ciphertext)
        .chain(&value.tag)
    {
        eval_byte(builder, *byte);
    }
}
