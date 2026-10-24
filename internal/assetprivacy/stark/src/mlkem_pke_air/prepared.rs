// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    mlkem_dot_air::{DotInputs, MlKemDotAir},
    mlkem_field::Q,
    mlkem_host::{EncryptionWitness, Polynomial, encode_ciphertext},
    mlkem_ntt_air::MlKemNttAir,
};

pub(super) struct Prepared<'a> {
    pub input: &'a EncryptionWitness,
    pub dot_input: DotInputs,
    pub dots: [Polynomial; 4],
    pub raw: [Polynomial; 4],
}

impl<'a> Prepared<'a> {
    pub fn new(input: &'a EncryptionWitness) -> Option<Self> {
        validate(input)?;
        let dot_input = dot_input(input)?;
        let dots = MlKemDotAir.values(&dot_input)?;
        let raw = inverse(&dots)?;
        check_outputs(input, &raw)?;
        Some(Self {
            input,
            dot_input,
            dots,
            raw,
        })
    }
}

fn validate(input: &EncryptionWitness) -> Option<()> {
    let values = input
        .matrix
        .iter()
        .flatten()
        .chain(&input.public_polynomials)
        .chain(&input.noise)
        .flatten();
    if values.copied().any(|value| value >= Q)
        || input.message.iter().any(|&v| !matches!(v, 0 | 1665))
    {
        return None;
    }
    Some(())
}

fn check_outputs(input: &EncryptionWitness, raw: &[Polynomial; 4]) -> Option<()> {
    for (slot, polynomial) in raw.iter().enumerate() {
        for (index, &value) in polynomial.iter().enumerate() {
            let mu = if slot == 3 { input.message[index] } else { 0 };
            let output = (value + input.noise[3 + slot][index] + mu) % Q;
            let claimed = if slot == 3 {
                input.v[index]
            } else {
                input.u[slot][index]
            };
            if output != claimed {
                return None;
            }
        }
    }
    (encode_ciphertext(&input.u, input.v)? == input.ciphertext).then_some(())
}

fn dot_input(input: &EncryptionWitness) -> Option<DotInputs> {
    let mut r_hat = [[0; 256]; 3];
    for (output, &noise) in r_hat.iter_mut().zip(&input.noise) {
        *output = MlKemNttAir::forward().values(noise)?;
    }
    if r_hat != input.r_hat {
        return None;
    }
    Some(DotInputs {
        matrix: input.matrix,
        public_polynomials: input.public_polynomials,
        r_hat,
    })
}

fn inverse(dots: &[Polynomial; 4]) -> Option<[Polynomial; 4]> {
    let mut raw = [[0; 256]; 4];
    for (output, &dot) in raw.iter_mut().zip(dots) {
        *output = MlKemNttAir::inverse().values(dot)?;
    }
    Some(raw)
}
