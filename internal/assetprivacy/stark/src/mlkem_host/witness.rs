// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{EncryptionWitness, encode, key::ParsedKey, noise, polynomial};
use crate::mlkem_encoding_air::MlKemEncodingAir;

pub(super) fn encrypt(
    key: ParsedKey,
    message: [u8; 32],
    coins: [u8; 32],
) -> Option<EncryptionWitness> {
    let mut witness = initial(key, message, coins)?;
    witness.r_hat = noise::transform(&witness.noise)?;
    witness.u = polynomial::vector(&witness.matrix, &witness.r_hat, &witness.noise)?;
    witness.v = polynomial::scalar(
        &witness.public_polynomials,
        &witness.r_hat,
        witness.noise[6],
        witness.message,
    )?;
    witness.ciphertext = encode::ciphertext(&witness.u, witness.v)?;
    Some(witness)
}

fn initial(key: ParsedKey, message: [u8; 32], coins: [u8; 32]) -> Option<EncryptionWitness> {
    Some(EncryptionWitness {
        public_polynomials: key.polynomials,
        matrix_seed: key.seed,
        matrix: key.matrix,
        noise: noise::sample(coins)?,
        r_hat: [[0; 256]; 3],
        message: MlKemEncodingAir::new(1)?.decode(&message)?,
        u: [[0; 256]; 3],
        v: [0; 256],
        ciphertext: [0; 1088],
    })
}
