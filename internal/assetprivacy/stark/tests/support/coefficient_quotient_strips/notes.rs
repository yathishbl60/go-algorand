// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Config, compact, current, fixture, notes_fixture, scheduler::CheckedStrips};
use p3_air::check_constraints;
use p3_batch_stark::{BatchProof, CommonData, ProverData, StarkInstance};
use p3_batch_stark::{prove_batch_with_quotient_scheduler, verify_batch};

type ResultData = (
    super::Air,
    BatchProof<compact::Config>,
    CommonData<compact::Config>,
);

pub fn run() {
    current::height_gate();
    verify_notes(1, 512, true);
    verify_notes(2, 1024, true);
    verify_notes(1, 512, false);
    verify_notes(2, 1024, false);
}

pub fn full_height() {
    let (air, proof, common) = prove(2, 65536, false);
    assert_eq!(proof.degree_bits, [17]);
    let opened = &proof.opened_values.instances[0];
    assert_eq!(opened.base_opened_values.quotient_chunks.len(), 16);
    assert_eq!(opened.permutation_local.len(), 20);
    let stock = compact::prover_config().unwrap();
    let public = vec![air.public_values().to_vec()];
    verify_batch(&stock, std::slice::from_ref(&air), &proof, &public, &common).unwrap();
    let changed = notes_fixture::air(1, 65536);
    assert!(verify_batch(&stock, &[changed], &proof, &public, &common).is_err());
}

fn verify_notes(count: usize, rows: usize, differential: bool) {
    let (air, proof, common) = prove(count, rows, differential);
    let stock = compact::prover_config().unwrap();
    let public = vec![air.public_values().to_vec()];
    verify_batch(&stock, std::slice::from_ref(&air), &proof, &public, &common).unwrap();
    let changed = notes_fixture::air(count, rows * 2);
    assert!(verify_batch(&stock, &[changed], &proof, &public, &common).is_err());
    if count == 2 {
        let changed = notes_fixture::air(1, rows);
        assert!(verify_batch(&stock, &[changed], &proof, &public, &common).is_err());
    }
}

fn prove(count: usize, rows: usize, differential: bool) -> ResultData {
    let air = notes_fixture::air(count, rows);
    let trace = trace(&air);
    let config = fixture::config(rows, differential);
    let instances = [StarkInstance {
        air: &air,
        trace: &trace,
        public_values: air.public_values().to_vec(),
    }];
    let data = ProverData::from_instances(&config, &instances).unwrap();
    assert!(data.common.preprocessed.is_none());
    let scheduler = CheckedStrips { differential };
    let proof =
        prove_batch_with_quotient_scheduler(&config, &instances, &data, &scheduler).unwrap();
    current::verify(&config, &air, &proof, &data.common);
    let common = CommonData::new(None, data.common.lookups);
    (air, stock_proof(proof), common)
}

fn trace(air: &super::Air) -> p3_matrix::dense::RowMajorMatrix<super::V> {
    let (notes, secrets) = notes_fixture::inputs(air.note_count(), 0);
    let trace = air.trace(&notes, &secrets).unwrap();
    if air.rows() <= 1024 {
        check_constraints(air, &trace, air.public_values());
    }
    trace
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
