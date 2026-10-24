// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Polynomial;
use crate::{mlkem_encoding_air::MlKemEncodingAir, mlkem_matrix_air::MlKemMatrixAir};

pub(super) struct ParsedKey {
    pub polynomials: [Polynomial; 3],
    pub seed: [u8; 32],
    pub matrix: [[Polynomial; 3]; 3],
}

impl ParsedKey {
    pub(super) fn parse(encoded: &[u8]) -> Option<Self> {
        if encoded.len() != 1184 {
            return None;
        }
        let encoding = MlKemEncodingAir::new(12)?;
        let mut polynomials = [[0; 256]; 3];
        for (index, polynomial) in polynomials.iter_mut().enumerate() {
            *polynomial = encoding.decode(&encoded[384 * index..384 * (index + 1)])?;
        }
        let seed = encoded[1152..].try_into().ok()?;
        Some(Self {
            polynomials,
            seed,
            matrix: sample(seed)?,
        })
    }
}

fn sample(seed: [u8; 32]) -> Option<[[Polynomial; 3]; 3]> {
    let mut matrix = [[[0; 256]; 3]; 3];
    for (row, polynomials) in matrix.iter_mut().enumerate() {
        for (column, polynomial) in polynomials.iter_mut().enumerate() {
            *polynomial = MlKemMatrixAir::candidate(column as u8, row as u8)?.values(seed)?;
        }
    }
    Some(matrix)
}
