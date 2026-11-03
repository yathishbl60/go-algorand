// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    coefficient_pcs::{self, Error, Limits},
    fixture,
};
use p3_commit::Pcs;
use p3_uni_stark::StarkGenericConfig;

pub fn run() {
    let mut limits = fixture::limits();
    limits.max_tile_values = 8192 * 10;
    rejected_commit(limits);
    let mut limits = fixture::limits();
    limits.max_coefficient_values = 1024 * 11 - 1;
    rejected_commit(limits);
    let mut limits = fixture::limits();
    limits.max_query_values = 192 * 11 - 1;
    rejected_commit(limits);
    let mut limits = fixture::limits();
    limits.max_claim_values = 10 * 11 - 1;
    rejected_commit(limits);
    opening_budget(1, 1_000_000);
    opening_budget(100_000_000, 110);
}

fn rejected_commit(limits: Limits) {
    let config = coefficient_pcs::prover_config(limits).unwrap();
    let domain = fixture::domain(&config, 512);
    let result = config.pcs().commit([(domain, fixture::matrix(512, 1, 1))]);
    assert!(matches!(result, Err(Error::Limits)));
}

fn opening_budget(evaluations: usize, claims: usize) {
    let mut limits = fixture::limits();
    limits.max_evaluation_values = evaluations;
    limits.max_claim_values = claims;
    let config = coefficient_pcs::prover_config(limits).unwrap();
    let (_, data) = fixture::commit(&config, 512, 1, 1);
    let mut challenger = config.initialise_challenger();
    let before = challenger.clone();
    let result = config
        .pcs()
        .open(vec![fixture::request(&data, 2)], &mut challenger);
    assert!(matches!(result, Err(Error::Limits)));
    fixture::unchanged(&before, &challenger);
    fixture::open(&fixture::config(), &data, 1, &mut challenger);
}
