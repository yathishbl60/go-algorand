// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MlKemNttAir;
use crate::mlkem_field::Q;

pub(super) struct Operation {
    pub left: usize,
    pub right: Option<usize>,
    pub zeta: u16,
    pub scale: bool,
}

impl MlKemNttAir {
    pub(super) fn operation(&self, step: usize) -> Option<Operation> {
        if step < 896 {
            return Some(self.butterfly(step));
        }
        if self.inverse && step < 1152 {
            return Some(Operation {
                left: step - 896,
                right: None,
                zeta: 3303,
                scale: true,
            });
        }
        None
    }

    fn butterfly(&self, step: usize) -> Operation {
        let stage = step / 128;
        let length = if self.inverse {
            2 << stage
        } else {
            128 >> stage
        };
        let group = (step % 128) / length;
        let left = 2 * length * group + step % length;
        Operation {
            left,
            right: Some(left + length),
            zeta: self.twiddle(stage, group),
            scale: false,
        }
    }
    fn twiddle(&self, stage: usize, group: usize) -> u16 {
        let index = if self.inverse {
            (128 >> stage) - 1 - group
        } else {
            (1 << stage) + group
        };
        root((index as u8).reverse_bits() >> 1)
    }
}

// FIPS 203 twiddles are 17^BitRev7(k) mod 3329, without Montgomery factors.
const fn root(mut exponent: u8) -> u16 {
    let mut value = 1u32;
    let mut power = 17u32;
    while exponent != 0 {
        if exponent & 1 != 0 {
            value = value * power % Q as u32;
        }
        power = power * power % Q as u32;
        exponent >>= 1;
    }
    value as u16
}
