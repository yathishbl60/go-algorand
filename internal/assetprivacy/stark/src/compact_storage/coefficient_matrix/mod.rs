// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Honest virtual bit-reversed LDE access and bounded multiquery FFT gathering.
mod access;
mod bridge;
mod gather;

use super::MaskedTrace;
use crate::compact::Value32;
use p3_field::{PrimeCharacteristicRing, TwoAdicField, coset::TwoAdicMultiplicativeCoset};
use std::borrow::Borrow;

/// Owns or borrows private masked coefficients; logical rows contain actual evaluations.
/// No expanded matrix, implicit cloning, Debug or serialization is provided.
pub struct CoefficientMatrix<S: Borrow<MaskedTrace<Value32>> + Send + Sync> {
    source: S,
    domain: TwoAdicMultiplicativeCoset<Value32>,
    rows: usize,
}

impl<S: Borrow<MaskedTrace<Value32>> + Send + Sync> CoefficientMatrix<S> {
    pub fn new(source: S, domain: TwoAdicMultiplicativeCoset<Value32>) -> Option<Self> {
        let rows = geometry(source.borrow(), domain)?;
        Some(Self {
            source,
            domain,
            rows,
        })
    }

    /// Private masked coefficient source, usable by bounded backend fast paths.
    pub fn mask(&self) -> &MaskedTrace<Value32> {
        self.source.borrow()
    }
}

fn geometry(
    source: &MaskedTrace<Value32>,
    domain: TwoAdicMultiplicativeCoset<Value32>,
) -> Option<usize> {
    let log = domain.log_size();
    let coefficient_log = source.coefficient_rows().ilog2() as usize;
    let rows = 1usize.checked_shl(u32::try_from(log).ok()?)?;
    if domain.shift() == Value32::ZERO || !valid_roots(log, coefficient_log) {
        return None;
    }
    rows.checked_mul(source.width())?;
    Some(rows)
}

fn valid_roots(log: usize, coefficient_log: usize) -> bool {
    log <= Value32::TWO_ADICITY
        && log >= coefficient_log
        && Value32::two_adic_generator(log).exp_power_of_2(log - coefficient_log)
            == Value32::two_adic_generator(coefficient_log)
}
