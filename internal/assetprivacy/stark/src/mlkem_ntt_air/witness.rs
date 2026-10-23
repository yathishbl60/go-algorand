// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemNttAir, program::Operation};
use crate::mlkem_field::Q;

pub(super) struct Arithmetic {
    pub left: u16,
    pub right: u16,
    pub product: u16,
    pub quotient: u16,
    pub low: u16,
    pub high: u16,
    pub carry: bool,
    pub borrow: bool,
}

impl MlKemNttAir {
    pub(super) fn arithmetic(&self, state: &[u16; 256], op: Option<&Operation>) -> Arithmetic {
        let Some(op) = op else {
            return arithmetic(0, 0, 0, 0);
        };
        let left = state[op.left];
        let right = op.right.map_or(0, |i| state[i]);
        if op.scale {
            let product = u32::from(left) * u32::from(op.zeta);
            return arithmetic(left, 0, 0, product);
        }
        if self.inverse {
            return inverse(left, right, op.zeta);
        }
        forward(left, right, op.zeta)
    }
}

fn forward(left: u16, right: u16, zeta: u16) -> Arithmetic {
    let product = u32::from(right) * u32::from(zeta);
    let reduced = (product % u32::from(Q)) as u16;
    let mut value = arithmetic(left, right, reduced, product);
    value.low = (u32::from(left + reduced) % u32::from(Q)) as u16;
    value.high = ((u32::from(left) + u32::from(Q) - u32::from(reduced)) % u32::from(Q)) as u16;
    value.carry = left + reduced >= Q;
    value.borrow = left < reduced;
    value
}

fn arithmetic(left: u16, right: u16, product: u16, expanded: u32) -> Arithmetic {
    Arithmetic {
        left,
        right,
        product,
        quotient: (expanded / u32::from(Q)) as u16,
        low: (expanded % u32::from(Q)) as u16,
        high: 0,
        carry: false,
        borrow: false,
    }
}

fn inverse(left: u16, right: u16, zeta: u16) -> Arithmetic {
    let difference = ((u32::from(right) + u32::from(Q) - u32::from(left)) % u32::from(Q)) as u16;
    let expanded = u32::from(zeta) * u32::from(difference);
    Arithmetic {
        left,
        right,
        product: difference,
        quotient: (expanded / u32::from(Q)) as u16,
        low: (u32::from(left + right) % u32::from(Q)) as u16,
        high: (expanded % u32::from(Q)) as u16,
        carry: left + right >= Q,
        borrow: right < left,
    }
}
