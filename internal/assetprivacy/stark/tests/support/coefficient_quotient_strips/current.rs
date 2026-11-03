// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Air, Config, fixture};
use p3_batch_stark::{BatchProof, CommonData, verify_batch};
use p3_commit::UnivariateStarkPcs;
use p3_uni_stark::{StarkGenericConfig, validate_degree_bits};

pub fn height_gate() {
    let config = fixture::config(512, false);
    let pcs = config.pcs();
    let min = pcs.log_min_trace_height();
    let max = pcs.log_max_trace_height();
    assert_eq!(
        validate_degree_bits(None, 17, 1, min, max).unwrap(),
        (16, 1 << 17)
    );
    assert!(validate_degree_bits(None, 18, 1, min, max).is_err());
}

pub fn verify(config: &Config, air: &Air, proof: &BatchProof<Config>, common: &CommonData<Config>) {
    let opened = &proof.opened_values.instances[0];
    assert_eq!(opened.permutation_local.len(), 20);
    verify_batch(
        config,
        std::slice::from_ref(air),
        proof,
        &[air.public_values().to_vec()],
        common,
    )
    .unwrap();
}
