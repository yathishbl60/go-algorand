// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

mod configuration;
pub mod fixture;
pub mod negative;
pub mod proving;
mod reduction;
pub mod verification;

use algorand_asset_privacy_stark::compact::{Challenge32 as E, CompactHash, Value32 as V};
use algorand_asset_privacy_stark::compact_storage::{
    CoefficientInputData, InputMmcs, MaskedTrace, TreeCap,
};
use p3_challenger::{HashChallenger, SerializingChallenger32};
use p3_commit::{CommitmentOpening, ExtensionMmcs};
use p3_field::coset::TwoAdicMultiplicativeCoset;
use p3_fri::{BatchMultiOpening, FriParameters, FriProof, TwoAdicFriFoldingForMmcs};
use p3_merkle_tree::MerkleTreeHidingMmcs;
use p3_symmetric::{CompressionFunctionFromHasher, SerializingHasher};
use rand::rngs::StdRng;
use std::sync::Arc;

type Domain = TwoAdicMultiplicativeCoset<V>;
type Leaf = SerializingHasher<CompactHash<1>>;
type Compress = CompressionFunctionFromHasher<CompactHash<2>, 2, 8>;
type Stock = MerkleTreeHidingMmcs<V, u64, Leaf, Compress, StdRng, 2, 8, 10>;
type FriMmcs = ExtensionMmcs<V, E, Stock>;
type Challenger = SerializingChallenger32<V, HashChallenger<u8, CompactHash<3>, 64>>;
type Proof = FriProof<E, FriMmcs, V, Vec<BatchMultiOpening<V, InputMmcs>>>;
type Claims = Vec<CommitmentOpening<E, TreeCap, Domain>>;
type Folding = TwoAdicFriFoldingForMmcs<V, InputMmcs>;

const LOG_ROWS: usize = 13;
const ROWS: usize = 1 << LOG_ROWS;

pub struct Fixture {
    source: Arc<MaskedTrace<V>>,
    input: InputMmcs,
    data: CoefficientInputData,
    cap: TreeCap,
    params: FriParameters<FriMmcs>,
    evaluation: Domain,
}

pub struct Argument {
    proof: Proof,
    values: Vec<E>,
}
