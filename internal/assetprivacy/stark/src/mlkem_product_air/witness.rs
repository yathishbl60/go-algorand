// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::mlkem_field::Q;

pub(super) struct Pair {
    pub operands: [u16; 4],
    pub gamma: u16,
    pub products: [(u16, u16); 5],
    pub sums: [(u16, bool); 2],
}

impl Pair {
    pub fn new(left: [u16; 2], right: [u16; 2], gamma: u16) -> Self {
        let p11 = multiply(left[1], right[1]);
        let gp11 = multiply(p11.0, gamma);
        let p00 = multiply(left[0], right[0]);
        let p01 = multiply(left[0], right[1]);
        let p10 = multiply(left[1], right[0]);
        Self {
            operands: [left[0], left[1], right[0], right[1]],
            gamma,
            products: [p11, gp11, p00, p01, p10],
            sums: [add(p00.0, gp11.0), add(p01.0, p10.0)],
        }
    }
}

fn multiply(left: u16, right: u16) -> (u16, u16) {
    let product = u32::from(left) * u32::from(right);
    (
        (product % u32::from(Q)) as u16,
        (product / u32::from(Q)) as u16,
    )
}

fn add(left: u16, right: u16) -> (u16, bool) {
    let sum = left + right;
    (sum % Q, sum >= Q)
}
