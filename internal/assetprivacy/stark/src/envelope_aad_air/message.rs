// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{EPOCH, EnvelopeAadAir, WINDOW};
use crate::Value;
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

impl EnvelopeAadAir {
    pub(super) fn message_bit<AB: AirBuilder<F = Value>>(
        &self,
        row: &[AB::Var],
        block: usize,
        bit: usize,
    ) -> AB::Expr {
        let global = block * 576 + bit;
        if global < 2048 {
            return self.context_bit::<AB>(row, global);
        }
        if global < 11520 {
            return row[WINDOW + global - 2048 - key_start(block)].into();
        }
        if global < 20224 {
            return row[WINDOW + bit].into();
        }
        AB::Expr::from_bool(matches!(global, 20225 | 20226 | 20735))
    }
    fn context_bit<AB: AirBuilder<F = Value>>(&self, row: &[AB::Var], bit: usize) -> AB::Expr {
        let byte = bit / 8;
        if (147..155).contains(&byte) {
            return row[EPOCH + bit - 147 * 8].into();
        }
        let index = byte - 8 * usize::from(byte >= 155);
        AB::Expr::from_bool(self.context[index] & (1 << (bit % 8)) != 0)
    }
}

pub(super) fn key_start(block: usize) -> usize {
    (block * 576).saturating_sub(2048) / 12 * 12
}

pub(super) fn bits<AB: AirBuilder<F = Value>>(bits: &[AB::Var]) -> AB::Expr {
    bits.iter()
        .rev()
        .fold(AB::Expr::ZERO, |value, &bit| value.double() + bit)
}
