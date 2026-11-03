// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{FriMmcs, StockInput};
use super::{E, V, fri};
use p3_fri::{BatchMultiOpening, FriProof};

type StockInner = FriProof<E, FriMmcs, V, Vec<BatchMultiOpening<V, StockInput>>>;

/// The proof protocol and MMCS multiproofs are identical; move fields without recoding.
pub(super) fn stock(proof: fri::Proof) -> StockInner {
    let input_openings = proof
        .input_openings
        .into_iter()
        .map(|opening| BatchMultiOpening {
            opened_values: opening.opened_values,
            opening_proof: opening.opening_proof,
        })
        .collect();
    FriProof {
        batch_pow_witness: proof.batch_pow_witness,
        commit_phase_commits: proof.commit_phase_commits,
        commit_pow_witnesses: proof.commit_pow_witnesses,
        input_openings,
        commit_phase_openings: proof.commit_phase_openings,
        final_poly: proof.final_poly,
        query_pow_witness: proof.query_pow_witness,
    }
}
