// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Air, Trace, context, fixture};
use algorand_asset_privacy_stark::compact::{Challenge32, Config, prover_config};
use p3_air::{BaseAir, check_constraints};
use p3_batch_stark::{BatchProof, ProverData, StarkInstance, prove_batch, verify_batch};
use p3_field::PrimeCharacteristicRing;
use p3_lookup::{LogUpGadget, LookupProtocol};

pub fn round_trip(count: usize, rows: usize) {
    let air = fixture::air(count, rows);
    let (notes, secrets) = fixture::inputs(count, 0);
    let trace = air
        .trace(&notes, &secrets)
        .expect("valid opening/secret pair");
    check_constraints(&air, &trace, air.public_values());
    let config = prover_config().expect("OS entropy");
    let (proof, data) = make(&config, &air, &trace);
    assert!(accepts(&config, &air, &proof, &data));
    geometry(&air, &proof, &data);
    context::reject(&config, &air, &proof, &data);
    let mut altered = proof;
    altered.opened_values.instances[0]
        .base_opened_values
        .trace_local[0] += Challenge32::ONE;
    assert!(!accepts(&config, &air, &altered, &data));
}

pub fn make(config: &Config, air: &Air, trace: &Trace) -> (BatchProof<Config>, ProverData<Config>) {
    let instances = [StarkInstance {
        air,
        trace,
        public_values: air.public_values().to_vec(),
    }];
    let data = ProverData::from_instances(config, &instances).expect("candidate setup");
    let proof = prove_batch(config, &instances, &data).expect("candidate proving");
    (proof, data)
}

pub fn accepts(
    config: &Config,
    air: &Air,
    proof: &BatchProof<Config>,
    data: &ProverData<Config>,
) -> bool {
    verify_batch(
        config,
        std::slice::from_ref(air),
        proof,
        &[air.public_values().to_vec()],
        &data.common,
    )
    .is_ok()
}

fn geometry(air: &Air, proof: &BatchProof<Config>, data: &ProverData<Config>) {
    assert_eq!((air.width(), air.num_periodic_columns()), (1988, 57));
    assert!(data.common.preprocessed.is_none());
    assert_eq!(data.common.lookups[0].len(), 1);
    assert_eq!(
        LogUpGadget::new().constraint_degree(&data.common.lookups[0][0]),
        8
    );
    assert_eq!(proof.lookup_terminals.len(), 1);
    assert_eq!(
        proof.lookup_terminals[0].as_ref().expect("one terminal").0,
        Challenge32::ZERO
    );
    let chunks = &proof.opened_values.instances[0]
        .base_opened_values
        .quotient_chunks;
    assert_eq!(chunks.len(), 16);
}
