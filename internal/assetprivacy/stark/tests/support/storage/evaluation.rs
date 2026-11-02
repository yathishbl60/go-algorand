// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Config, Domain, E, Matrix, V, fixture::Fixture};
use p3_air::symbolic::AirLayout;
use p3_batch_stark::{StarkGenericConfig, prover::quotient_values};
use p3_dft::{Radix2Dit, TwoAdicSubgroupDft};
use p3_field::PrimeCharacteristicRing;
use p3_lookup::LogUpGadget;

pub struct Quotient<'a> {
    pub config: &'a Config,
    pub air: &'a Fixture,
    pub trace: Domain<V>,
}

impl Quotient<'_> {
    pub fn evaluate(&self, domain: Domain<V>, values: &Matrix) -> Vec<E> {
        quotient_values::<Config, _, _, _>(
            self.config.pcs(),
            self.air,
            &[],
            AirLayout::from_air(self.air),
            self.trace,
            domain,
            values,
            None,
            &[],
            &[],
            &LogUpGadget::new(),
            &[],
            None,
            E::from_u8(13),
        )
    }
}

pub fn evaluate(coefficients: &Matrix, domain: Domain<V>) -> Matrix {
    let mut values = coefficients.values.clone();
    values.resize(domain.size() * coefficients.width, V::ZERO);
    Radix2Dit::default().coset_dft_batch(Matrix::new(values, coefficients.width), domain.shift())
}
