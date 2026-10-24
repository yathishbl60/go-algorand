// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Polynomial;
use crate::{mlkem_noise_air::MlKemNoiseAir, mlkem_ntt_air::MlKemNttAir};

pub(super) fn sample(coins: [u8; 32]) -> Option<[Polynomial; 7]> {
    let mut polynomials = [[0; 256]; 7];
    for (nonce, polynomial) in polynomials.iter_mut().enumerate() {
        *polynomial = MlKemNoiseAir::new(nonce as u8)?.values(coins);
    }
    Some(polynomials)
}

pub(super) fn transform(noise: &[Polynomial; 7]) -> Option<[Polynomial; 3]> {
    let mut transformed = [[0; 256]; 3];
    for (output, &input) in transformed.iter_mut().zip(noise) {
        *output = MlKemNttAir::forward().values(input)?;
    }
    Some(transformed)
}
