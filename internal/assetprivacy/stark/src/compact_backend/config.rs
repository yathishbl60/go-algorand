// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::hash::CompactHash;
use super::types::{ChallengeMmcs, Challenger, Compress, Config, LeafHash, Pcs, ValueMmcs};
use super::{LOG_BLOWUP, LOG_FINAL_POLY_LEN, MAX_LOG_ARITY, NUM_QUERIES, RANDOM_CODEWORDS};
use p3_challenger::HashChallenger;
use p3_fri::FriParameters;
use rand::{
    SeedableRng,
    rngs::{StdRng, SysError, SysRng},
};

/// Fresh independent OS-backed streams for leaf salts and private masks per call.
/// A complete proof relation and reviewed security parameters remain necessary.
pub fn prover_config() -> Result<Config, SysError> {
    let leaves = StdRng::try_from_rng(&mut SysRng)?;
    let masks = StdRng::try_from_rng(&mut SysRng)?;
    let leaves = value_mmcs(leaves);
    let extensions = ChallengeMmcs::new(leaves.clone());
    let pcs = Pcs::new(
        Default::default(),
        leaves,
        fri(extensions),
        RANDOM_CODEWORDS,
        masks,
    );
    let transcript = HashChallenger::new(b"NAP-UNIVERSAL-CANDIDATE-1".to_vec(), CompactHash::<3>);
    Ok(Config::new(pcs, Challenger::new(transcript)))
}

fn value_mmcs(rng: StdRng) -> ValueMmcs {
    ValueMmcs::new(
        LeafHash::new(CompactHash::<1>),
        Compress::new(CompactHash::<2>),
        0,
        rng,
    )
}

fn fri(mmcs: ChallengeMmcs) -> FriParameters<ChallengeMmcs> {
    FriParameters {
        log_blowup: LOG_BLOWUP,
        log_final_poly_len: LOG_FINAL_POLY_LEN,
        max_log_arity: MAX_LOG_ARITY,
        num_queries: NUM_QUERIES,
        batch_proof_of_work_bits: 0,
        commit_proof_of_work_bits: 0,
        query_proof_of_work_bits: 0,
        mmcs,
    }
}
