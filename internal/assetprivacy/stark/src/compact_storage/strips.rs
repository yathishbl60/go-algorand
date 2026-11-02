// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_field::{TwoAdicField, coset::TwoAdicMultiplicativeCoset};

pub const QUOTIENT_STRIP_COUNT: usize = 8;

/// Exact natural-order decomposition of a disjoint 16N quotient domain.
/// Each strip has 2N points; original trace constraints retain their N domain.
#[derive(Clone, Copy)]
pub struct QuotientStrips<F: TwoAdicField> {
    quotient: TwoAdicMultiplicativeCoset<F>,
    strip_log: usize,
    strip_rows: usize,
    total_rows: usize,
}

impl<F: TwoAdicField> QuotientStrips<F> {
    /// Rejects intersecting domains, incompatible geometry and usize overflow.
    pub fn new(
        trace: TwoAdicMultiplicativeCoset<F>,
        quotient: TwoAdicMultiplicativeCoset<F>,
    ) -> Option<Self> {
        let full_log = trace.log_size().checked_add(4)?;
        let total_rows = 1usize.checked_shl(u32::try_from(full_log).ok()?)?;
        if !valid_domains(trace, quotient, full_log) {
            return None;
        }
        Some(Self {
            quotient,
            strip_log: full_log.checked_sub(3)?,
            strip_rows: total_rows / QUOTIENT_STRIP_COUNT,
            total_rows,
        })
    }

    pub fn strip_rows(&self) -> usize {
        self.strip_rows
    }

    pub fn total_rows(&self) -> usize {
        self.total_rows
    }

    /// Strip j begins at shift*g_16N^j and advances by g_16N^8.
    pub fn strip(&self, index: usize) -> Option<TwoAdicMultiplicativeCoset<F>> {
        if index >= QUOTIENT_STRIP_COUNT {
            return None;
        }
        let exponent = u64::try_from(index).ok()?;
        let shift = self.quotient.shift() * self.quotient.subgroup_generator().exp_u64(exponent);
        TwoAdicMultiplicativeCoset::new(shift, self.strip_log)
    }

    /// Q[j+8*r] receives row r of strip j; no bit-reversal is implied here.
    pub fn global_row(&self, strip: usize, row: usize) -> Option<usize> {
        if strip >= QUOTIENT_STRIP_COUNT || row >= self.strip_rows {
            return None;
        }
        row.checked_mul(QUOTIENT_STRIP_COUNT)?.checked_add(strip)
    }
}

fn valid_domains<F: TwoAdicField>(
    trace: TwoAdicMultiplicativeCoset<F>,
    quotient: TwoAdicMultiplicativeCoset<F>,
    full_log: usize,
) -> bool {
    trace.shift() != F::ZERO
        && quotient.shift() != F::ZERO
        && full_log <= F::TWO_ADICITY
        && quotient.log_size() == full_log
        && !quotient.contains(trace.shift())
        && quotient.subgroup_generator().exp_power_of_2(4) == trace.subgroup_generator()
        && quotient.subgroup_generator().exp_power_of_2(3) == F::two_adic_generator(full_log - 3)
}
