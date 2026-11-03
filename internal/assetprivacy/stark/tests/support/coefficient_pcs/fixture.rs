// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Cap, Challenger, Config, Data, Domain, E, V, coefficient_pcs};
use p3_challenger::{CanObserve, FieldChallenger};
use p3_commit::Pcs;
use p3_field::{BasedVectorSpace, PrimeCharacteristicRing};
use p3_matrix::dense::RowMajorMatrix;
use p3_uni_stark::StarkGenericConfig;

pub fn limits() -> coefficient_pcs::Limits {
    coefficient_pcs::Limits {
        tile_columns: 16,
        max_tile_values: 4_194_304,
        max_coefficient_values: 20_000_000,
        max_evaluation_values: 100_000_000,
        max_claim_values: 1_000_000,
        max_query_values: 5_000_000,
    }
}

pub fn config() -> Config {
    coefficient_pcs::prover_config(limits()).unwrap()
}

pub fn domain(config: &Config, rows: usize) -> Domain {
    config.pcs().natural_domain_for_degree(rows * 2)
}

pub fn matrix(rows: usize, width: usize, seed: usize) -> RowMajorMatrix<V> {
    RowMajorMatrix::new(
        (0..rows * width)
            .map(|i| V::from_usize((i * 17 + seed) % 65521))
            .collect(),
        width,
    )
}

pub fn point(seed: usize) -> E {
    E::from_basis_coefficients_fn(|i| V::from_usize(seed + i * 13 + 1))
}

pub fn challenger(config: &Config, caps: &[Cap], context: usize) -> Challenger {
    let mut challenger = config.initialise_challenger();
    challenger.observe(V::from_usize(context));
    for cap in caps {
        challenger.observe(cap.clone());
    }
    challenger
}

pub fn unchanged(before: &Challenger, after: &Challenger) {
    let mut before = before.clone();
    let mut after = after.clone();
    assert_eq!(
        before.sample_algebra_element::<E>(),
        after.sample_algebra_element::<E>()
    );
}

pub fn commit(config: &Config, rows: usize, width: usize, seed: usize) -> (Cap, Data) {
    config
        .pcs()
        .commit([(domain(config, rows), matrix(rows, width, seed))])
        .unwrap()
}

pub fn request(data: &Data, count: usize) -> p3_commit::OpeningRequest<'_, Data, E> {
    (data, vec![(0..count).map(point).collect()]).into()
}

pub fn open(config: &Config, data: &Data, count: usize, challenger: &mut Challenger) {
    config
        .pcs()
        .open(vec![request(data, count)], challenger)
        .unwrap();
}
