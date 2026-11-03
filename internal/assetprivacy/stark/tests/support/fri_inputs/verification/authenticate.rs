// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Challenger, E, Fixture, LOG_ROWS, Proof, ROWS, V, configuration};
use p3_commit::Mmcs;
use p3_fri::{FriShape, VerifierTranscript};
use p3_matrix::Dimensions;

pub fn indices(f: &Fixture, proof: &Proof, challenger: &mut Challenger) -> Vec<usize> {
    let shape = FriShape::new(&f.params, &[LOG_ROWS], LOG_ROWS);
    let mut transcript = VerifierTranscript::<Challenger, V, E>::new(challenger, shape);
    transcript
        .commit_rounds(&proof.commit_phase_commits, &proof.commit_pow_witnesses)
        .expect("unchanged commitment phases");
    let indices = transcript
        .query_phase(&proof.final_poly, Some(proof.query_pow_witness))
        .expect("zero proof-of-work");
    transcript.finish();
    indices
}

pub fn input(f: &Fixture, proof: &Proof, indices: &[usize]) -> bool {
    let opening = &proof.input_openings[0];
    configuration::stock()
        .verify_multi_batch(
            &f.cap,
            &[Dimensions {
                width: 11,
                height: ROWS,
            }],
            indices,
            &opening.opened_values,
            &opening.opening_proof,
        )
        .is_ok()
}
