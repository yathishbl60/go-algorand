// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AAD_ROW, EnvelopeAir, WORK, prepared::Prepared, target::shift};
use crate::Value;

pub(super) fn write(
    air: &EnvelopeAir,
    input: &Prepared<'_>,
    values: &mut [Value],
    shape: (usize, usize, usize),
) -> Option<[u8; 64]> {
    air.aad.write_into(
        input.input.public_key,
        input.input.epoch,
        input.kem.encryption.ciphertext,
        values,
        shift(shape, WORK, AAD_ROW),
    )
}
