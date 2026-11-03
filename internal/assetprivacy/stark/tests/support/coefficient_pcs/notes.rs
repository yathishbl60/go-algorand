// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Config, compact, fixture, notes_fixture};
use p3_air::check_constraints;
use p3_batch_stark::{
    BatchProof, CommonData, ProverData, StarkInstance, prove_batch, verify_batch,
};

pub fn run() {
    prove_owned_notes(1, 512);
    prove_owned_notes(2, 1024);
}

fn prove_owned_notes(count: usize, rows: usize) {
    let (air, proof, common) = prove(count, rows);
    let stock = compact::prover_config().unwrap();
    let public = vec![air.public_values().to_vec()];
    verify_batch(&stock, std::slice::from_ref(&air), &proof, &public, &common).unwrap();
    let changed = notes_fixture::air(count, rows * 2);
    assert!(verify_batch(&stock, &[changed], &proof, &public, &common).is_err());
}

type ResultData = (
    super::Air,
    BatchProof<compact::Config>,
    CommonData<compact::Config>,
);

fn prove(count: usize, rows: usize) -> ResultData {
    let air = notes_fixture::air(count, rows);
    let (notes, secrets) = notes_fixture::inputs(count, 0);
    let trace = air.trace(&notes, &secrets).unwrap();
    check_constraints(&air, &trace, air.public_values());
    let config = fixture::config();
    let instances = [StarkInstance {
        air: &air,
        trace: &trace,
        public_values: air.public_values().to_vec(),
    }];
    let data = ProverData::from_instances(&config, &instances).unwrap();
    assert!(data.common.preprocessed.is_none());
    let proof = prove_batch(&config, &instances, &data).unwrap();
    let common = CommonData::new(None, data.common.lookups);
    let proof = stock_proof(proof);
    (air, proof, common)
}

fn stock_proof(proof: BatchProof<Config>) -> BatchProof<compact::Config> {
    BatchProof {
        commitments: proof.commitments,
        opened_values: proof.opened_values,
        opening_proof: proof.opening_proof,
        lookup_terminals: proof.lookup_terminals,
        degree_bits: proof.degree_bits,
        lookup_pow_witness: proof.lookup_pow_witness,
        ood_pow_witness: proof.ood_pow_witness,
    }
}
