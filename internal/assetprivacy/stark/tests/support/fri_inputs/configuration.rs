// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Challenger, CompactHash, Compress, FriMmcs, Leaf, Stock, TreeCap, V};
use algorand_asset_privacy_stark::compact::{
    LOG_BLOWUP, LOG_FINAL_POLY_LEN, MAX_LOG_ARITY, NUM_QUERIES,
};
use p3_challenger::{CanObserve, HashChallenger};
use p3_field::PrimeCharacteristicRing;
use p3_fri::FriParameters;
use rand::{
    SeedableRng,
    rngs::{StdRng, SysRng},
};

pub fn entropy() -> StdRng {
    StdRng::try_from_rng(&mut SysRng).expect("OS entropy")
}

pub fn stock() -> Stock {
    Stock::new(
        Leaf::new(CompactHash::<1>),
        Compress::new(CompactHash::<2>),
        0,
        entropy(),
    )
}

pub fn params() -> FriParameters<FriMmcs> {
    FriParameters {
        log_blowup: LOG_BLOWUP,
        log_final_poly_len: LOG_FINAL_POLY_LEN,
        max_log_arity: MAX_LOG_ARITY,
        num_queries: NUM_QUERIES,
        batch_proof_of_work_bits: 0,
        commit_proof_of_work_bits: 0,
        query_proof_of_work_bits: 0,
        mmcs: FriMmcs::new(stock()),
    }
}

pub fn challenger(cap: &TreeCap, context: u64) -> Challenger {
    let transcript = HashChallenger::new(b"NAP-UNIVERSAL-CANDIDATE-1".to_vec(), CompactHash::<3>);
    let mut challenger = Challenger::new(transcript);
    challenger.observe(V::from_u64(context));
    challenger.observe(cap.clone());
    challenger
}
