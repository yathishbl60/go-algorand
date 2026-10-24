// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{PUBLIC, RHAT};
use crate::{mlkem_field::Q, mlkem_product_air::MlKemProductAir};

pub type Polynomial = [u16; 256];

/// Private prover inputs. No Debug or serialization is implemented.
pub struct DotInputs {
    pub matrix: [[Polynomial; 3]; 3],
    pub public_polynomials: [Polynomial; 3],
    pub r_hat: [Polynomial; 3],
}

impl DotInputs {
    pub(super) fn valid(&self) -> bool {
        self.matrix
            .iter()
            .flatten()
            .chain(&self.public_polynomials)
            .chain(&self.r_hat)
            .all(|p| p.iter().all(|&v| v < Q))
    }
    pub(super) fn left(&self, op: usize) -> &Polynomial {
        if op / 3 < 3 {
            &self.matrix[op % 3][op / 3]
        } else {
            &self.public_polynomials[op % 3]
        }
    }
    pub(super) fn right(&self, op: usize) -> &Polynomial {
        &self.r_hat[op % 3]
    }
    pub(super) fn dot(&self, index: usize) -> Option<Polynomial> {
        let mut output = [0; 256];
        for op in index * 3..index * 3 + 3 {
            let product = MlKemProductAir.values(*self.left(op), *self.right(op))?;
            for (sum, value) in output.iter_mut().zip(product) {
                *sum = (*sum + value) % Q;
            }
        }
        Some(output)
    }
}

pub(super) fn source(op: usize, right: bool) -> usize {
    if right {
        return RHAT + op % 3 * 256;
    }
    if op / 3 < 3 {
        (op % 3 * 3 + op / 3) * 256
    } else {
        PUBLIC + op % 3 * 256
    }
}
