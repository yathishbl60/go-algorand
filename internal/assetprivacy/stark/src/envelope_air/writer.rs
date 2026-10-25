// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    EnvelopeAir, GCM_ROW, HKDF_ROW, NOTE_ROW, WORK, prepared::Prepared, target::shift, wire,
};
use crate::{Value, gcm_vertical::GcmVerticalAir, mlkem_encapsulation_air::MlKemEncapsulationAir};

pub(super) fn write(
    air: &EnvelopeAir,
    input: &Prepared<'_>,
    values: &mut [Value],
    shape: (usize, usize, usize),
) -> Option<[u8; 1629]> {
    let kem = kem(input, values, shape)?;
    hashes(air, input, values, shape)?;
    let (ciphertext, tag) = cipher(input, values, shape)?;
    note(air, input, values, shape)?;
    Some(wire::encode(kem, input.input.nonce, ciphertext, tag))
}

fn hashes(
    air: &EnvelopeAir,
    input: &Prepared<'_>,
    values: &mut [Value],
    shape: (usize, usize, usize),
) -> Option<()> {
    let associated = super::writer_aad::write(air, input, values, shape)?;
    let key = air.hkdf.write_into(
        &input.kem.shared_key,
        &associated,
        values,
        shift(shape, WORK, HKDF_ROW),
    )?;
    (associated == input.associated && key == input.key).then_some(())
}

fn cipher(
    input: &Prepared<'_>,
    values: &mut [Value],
    shape: (usize, usize, usize),
) -> Option<([u8; 512], [u8; 16])> {
    GcmVerticalAir.write_into(
        input.key,
        input.input.nonce,
        input.associated,
        *input.input.opening,
        values,
        shift(shape, WORK, GCM_ROW),
    )
}

fn kem(
    input: &Prepared<'_>,
    values: &mut [Value],
    shape: (usize, usize, usize),
) -> Option<[u8; 1088]> {
    let key = input.input.public_key;
    let message = input.input.message;
    let result = MlKemEncapsulationAir.write_into(key, message, values, shift(shape, WORK, 0))?;
    let ciphertext = result.encryption.ciphertext;
    (ciphertext == input.kem.encryption.ciphertext).then_some(ciphertext)
}

fn note(
    air: &EnvelopeAir,
    input: &Prepared<'_>,
    values: &mut [Value],
    shape: (usize, usize, usize),
) -> Option<()> {
    let target = shift(shape, WORK, NOTE_ROW);
    let commitment = air.note.write_into(input.input.opening, values, target)?;
    (commitment == air.context.commitment()).then_some(())
}
