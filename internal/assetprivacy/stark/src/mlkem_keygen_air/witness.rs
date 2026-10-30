// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MlKemKeygenAir;
use crate::{
    mlkem_dot_air::{DotInputs, Polynomial},
    mlkem_field::Q,
    mlkem_keygen_seed_air::writer::expand,
    mlkem_noise_air::MlKemNoiseAir,
    mlkem_ntt_air::MlKemNttAir,
};

pub(super) struct Inputs {
    pub dot: DotInputs,
    pub transformed: [Polynomial; 6],
    pub carry: [u16; 768],
}

impl MlKemKeygenAir {
    pub(super) fn prepare(&self, seed: [u8; 32]) -> Option<Inputs> {
        let (_, expanded) = expand(seed);
        if expanded[..32] != self.key.rho {
            return None;
        }
        let sigma = expanded[32..].try_into().ok()?;
        let transformed = transforms(sigma)?;
        let dot = self.key.dot_input(transformed[..3].try_into().ok()?);
        let products = self.dot.values(&dot)?;
        let carry = self.carries(&products, &transformed)?;
        Some(Inputs {
            dot,
            transformed,
            carry,
        })
    }
    fn carries(
        &self,
        products: &[Polynomial; 4],
        transformed: &[Polynomial; 6],
    ) -> Option<[u16; 768]> {
        let mut carry = [0; 768];
        for (i, to) in carry.iter_mut().enumerate() {
            let sum = products[i / 256][i % 256] + transformed[3 + i / 256][i % 256];
            if sum % Q != self.key.polynomials[i / 256][i % 256] {
                return None;
            }
            *to = u16::from(sum >= Q);
        }
        Some(carry)
    }
}
fn transforms(sigma: [u8; 32]) -> Option<[Polynomial; 6]> {
    let mut values = [[0; 256]; 6];
    for (i, output) in values.iter_mut().enumerate() {
        let noise = MlKemNoiseAir::new(i as u8)?.values(sigma);
        *output = MlKemNttAir::forward().values(noise)?;
    }
    Some(values)
}
