// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{EnvelopeAir, EnvelopeInputs};
use crate::{
    hkdf_air::derive_key,
    mlkem_host::{Encapsulation, encapsulate},
};
use sha3::{Digest, Sha3_512};

pub(super) struct Prepared<'a> {
    pub input: EnvelopeInputs<'a>,
    pub kem: Encapsulation,
    pub associated: [u8; 64],
    pub key: [u8; 32],
}

impl<'a> Prepared<'a> {
    pub fn new(air: &EnvelopeAir, input: EnvelopeInputs<'a>) -> Option<Self> {
        if input.epoch == 0 || air.note.commitment(input.opening)? != air.context.commitment() {
            return None;
        }
        let kem = encapsulate(input.public_key, input.message)?;
        let context = air.context.encryption_bytes(input.epoch);
        let associated = associated(&context, input.public_key, &kem.encryption.ciphertext);
        let key = derive_key(&kem.shared_key, &associated);
        Some(Self {
            input,
            kem,
            associated,
            key,
        })
    }
}

fn associated(context: &[u8; 256], public_key: &[u8], ciphertext: &[u8; 1088]) -> [u8; 64] {
    let mut hash = Sha3_512::new();
    hash.update(context);
    hash.update(public_key);
    hash.update(ciphertext);
    hash.finalize().into()
}
