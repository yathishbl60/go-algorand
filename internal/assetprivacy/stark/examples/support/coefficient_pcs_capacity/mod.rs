// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

mod geometry;
mod opening;

use algorand_asset_privacy_stark::{coefficient_pcs as pcs, compact};
use p3_commit::Pcs;
use p3_field::coset::TwoAdicMultiplicativeCoset;
use p3_uni_stark::StarkGenericConfig;

type V = compact::Value32;
type E = compact::Challenge32;
type Config = pcs::Config;
type Challenger = <Config as StarkGenericConfig>::Challenger;
type Cap = <pcs::CoefficientPcs as Pcs<E, Challenger>>::Commitment;
type Proof = <pcs::CoefficientPcs as Pcs<E, Challenger>>::Proof;
type Domain = TwoAdicMultiplicativeCoset<V>;
type Claims = p3_commit::OpenedValues<E>;

struct Statement {
    cap: Cap,
    domain: Domain,
    points: Vec<E>,
}

pub fn run() {
    let config = pcs::prover_config(geometry::limits()).expect("OS entropy and valid limits");
    let (cap, data, domain) = geometry::commit(&config);
    let (statement, values, proof) = opening::prove(&config, cap, domain, &data);
    opening::verify(statement, values, &proof);
    println!(
        "status=full_dimension_pcs_verified complete_semantic_prover=0 native_transaction_accepted=0"
    );
}
