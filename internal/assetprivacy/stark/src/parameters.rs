// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_fri::FriParameters;

use crate::types::ChallengeMmcs;

// Candidate parameters; not an established quantum soundness bound.
pub const NUM_QUERIES: usize = 192;
pub const RANDOM_CODEWORDS: usize = 5;
// Hiding needs N >= 2 * (queries + extension_degree * opening_points).
// The final relation must check its actual opening count, including quotients.
pub const MIN_TRACE_ROWS: usize = 512;

pub(crate) const fn fri(mmcs: ChallengeMmcs) -> FriParameters<ChallengeMmcs> {
    FriParameters {
        log_blowup: 3,
        log_final_poly_len: 0,
        max_log_arity: 1,
        num_queries: NUM_QUERIES,
        batch_proof_of_work_bits: 0,
        commit_proof_of_work_bits: 0,
        query_proof_of_work_bits: 0,
        mmcs,
    }
}
