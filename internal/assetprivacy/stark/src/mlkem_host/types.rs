// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub type Polynomial = [u16; 256];

/// Private prover data. Deliberately has no Debug or serialization implementation.
pub struct Encapsulation {
    pub key_hash: [u8; 32],
    pub shared_key: [u8; 32],
    pub coins: [u8; 32],
    pub encryption: EncryptionWitness,
}

/// Private FIPS 203 Algorithm 14 intermediates; only ciphertext may be published.
pub struct EncryptionWitness {
    pub public_polynomials: [Polynomial; 3],
    pub matrix_seed: [u8; 32],
    pub matrix: [[Polynomial; 3]; 3],
    pub noise: [Polynomial; 7],
    pub r_hat: [Polynomial; 3],
    pub message: Polynomial,
    pub u: [Polynomial; 3],
    pub v: Polynomial,
    pub ciphertext: [u8; 1088],
}
