// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    mlkem_dot_air::{DotInputs, Polynomial},
    mlkem_encoding_air::MlKemEncodingAir,
    mlkem_matrix_air::MlKemMatrixAir,
};

#[derive(Clone)]
pub(super) struct PublicKey {
    pub encoded: [u8; 1184],
    pub rho: [u8; 32],
    pub polynomials: [Polynomial; 3],
    pub matrix: [[Polynomial; 3]; 3],
}

impl PublicKey {
    pub fn parse(encoded: [u8; 1184]) -> Option<Self> {
        let encoding = MlKemEncodingAir::new(12)?;
        let mut polynomials = [[0; 256]; 3];
        for (i, p) in polynomials.iter_mut().enumerate() {
            *p = encoding.decode(&encoded[i * 384..(i + 1) * 384])?;
        }
        let rho = encoded[1152..].try_into().ok()?;
        let matrix = matrix(rho)?;
        Some(Self {
            encoded,
            rho,
            polynomials,
            matrix,
        })
    }
    pub fn dot_input(&self, s_hat: [Polynomial; 3]) -> DotInputs {
        DotInputs {
            matrix: core::array::from_fn(|term| {
                core::array::from_fn(|output| self.matrix[output][term])
            }),
            public_polynomials: [[0; 256]; 3],
            r_hat: s_hat,
        }
    }
}

fn matrix(rho: [u8; 32]) -> Option<[[Polynomial; 3]; 3]> {
    let mut matrix = [[[0; 256]; 3]; 3];
    for (row, polynomials) in matrix.iter_mut().enumerate() {
        for (column, p) in polynomials.iter_mut().enumerate() {
            *p = MlKemMatrixAir::candidate(column as u8, row as u8)?.values(rho)?;
        }
    }
    Some(matrix)
}
