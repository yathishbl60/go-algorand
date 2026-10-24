// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MlKemNttAir;
use crate::mlkem_field::ntt_twiddle;

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

    // Stage zero touches every register once before it can be overwritten.
    // Selected-operand canonical bounds rely on this trusted schedule invariant.
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
        ntt_twiddle(index as u8).expect("fixed NTT twiddle index")
    }
}
