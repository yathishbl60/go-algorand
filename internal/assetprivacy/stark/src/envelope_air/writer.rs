// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    AAD_ROW, EnvelopeAir, GCM_ROW, HKDF_ROW, NOTE_ROW, WIDTH, WORK, prepared::Prepared, wire,
};
use crate::{Value, gcm_vertical::GcmVerticalAir, mlkem_encapsulation_air::MlKemEncapsulationAir};

pub(super) fn write(
    air: &EnvelopeAir,
    input: &Prepared<'_>,
    values: &mut [Value],
) -> Option<[u8; 1629]> {
    let kem = kem(input, values)?;
    hashes(air, input, values)?;
    let (ciphertext, tag) = cipher(input, values)?;
    note(air, input, values)?;
    Some(wire::encode(kem, input.input.nonce, ciphertext, tag))
}

fn hashes(air: &EnvelopeAir, input: &Prepared<'_>, values: &mut [Value]) -> Option<()> {
    let associated = air.aad.write_into(
        input.input.public_key,
        input.input.epoch,
        input.kem.encryption.ciphertext,
        values,
        (WIDTH, WORK, AAD_ROW),
    )?;
    let key = air.hkdf.write_into(
        &input.kem.shared_key,
        &associated,
        values,
        (WIDTH, WORK, HKDF_ROW),
    )?;
    (associated == input.associated && key == input.key).then_some(())
}

fn cipher(input: &Prepared<'_>, values: &mut [Value]) -> Option<([u8; 512], [u8; 16])> {
    GcmVerticalAir.write_into(
        input.key,
        input.input.nonce,
        input.associated,
        *input.input.opening,
        values,
        (WIDTH, WORK, GCM_ROW),
    )
}

fn kem(input: &Prepared<'_>, values: &mut [Value]) -> Option<[u8; 1088]> {
    let key = input.input.public_key;
    let message = input.input.message;
    let result = MlKemEncapsulationAir.write_into(key, message, values, (WIDTH, WORK, 0))?;
    let ciphertext = result.encryption.ciphertext;
    (ciphertext == input.kem.encryption.ciphertext).then_some(ciphertext)
}

fn note(air: &EnvelopeAir, input: &Prepared<'_>, values: &mut [Value]) -> Option<()> {
    let commitment = air
        .note
        .write_into(input.input.opening, values, (WIDTH, WORK, NOTE_ROW))?;
    (commitment == air.context.commitment()).then_some(())
}
