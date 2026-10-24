// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Polynomial;
use crate::{mlkem_field::Q, mlkem_ntt_air::MlKemNttAir, mlkem_product_air::MlKemProductAir};

pub(super) fn vector(
    matrix: &[[Polynomial; 3]; 3],
    r_hat: &[Polynomial; 3],
    noise: &[Polynomial; 7],
) -> Option<[Polynomial; 3]> {
    let mut output = [[0; 256]; 3];
    for (index, polynomial) in output.iter_mut().enumerate() {
        let column = core::array::from_fn(|row| matrix[row][index]);
        let product = MlKemNttAir::inverse().values(dot(&column, r_hat)?)?;
        *polynomial = add(product, noise[3 + index]);
    }
    Some(output)
}

pub(super) fn scalar(
    public: &[Polynomial; 3],
    r_hat: &[Polynomial; 3],
    error: Polynomial,
    message: Polynomial,
) -> Option<Polynomial> {
    let product = MlKemNttAir::inverse().values(dot(public, r_hat)?)?;
    Some(add(add(product, error), message))
}

fn dot(left: &[Polynomial; 3], right: &[Polynomial; 3]) -> Option<Polynomial> {
    let mut sum = [0; 256];
    for (&left, &right) in left.iter().zip(right) {
        sum = add(sum, MlKemProductAir.values(left, right)?);
    }
    Some(sum)
}

fn add(left: Polynomial, right: Polynomial) -> Polynomial {
    core::array::from_fn(|index| (left[index] + right[index]) % Q)
}
