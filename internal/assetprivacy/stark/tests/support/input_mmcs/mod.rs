// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

mod corruption;
pub mod dense;
mod fixture;
mod frontier;
pub mod malformed;
mod openings;
pub mod streamed;
mod verify;

use algorand_asset_privacy_stark::{
    compact::{CompactHash, Value32},
    compact_storage::{CoefficientInputData, InputMmcs, MaskedTrace, TreeCap},
};
use p3_commit::Mmcs;
use p3_field::coset::TwoAdicMultiplicativeCoset;
use p3_matrix::{Dimensions, dense::RowMajorMatrix};
use p3_merkle_tree::MerkleTreeHidingMmcs;
use p3_symmetric::{CompressionFunctionFromHasher, SerializingHasher};
use rand::rngs::StdRng;

type V = Value32;
type Dense = RowMajorMatrix<V>;
type Domain = TwoAdicMultiplicativeCoset<V>;
type Mask = MaskedTrace<V>;
type Leaf = SerializingHasher<CompactHash<1>>;
type Compress = CompressionFunctionFromHasher<CompactHash<2>, 2, 8>;
type Stock = MerkleTreeHidingMmcs<V, u64, Leaf, Compress, StdRng, 2, 8, 10>;
type Values = Vec<Vec<Vec<V>>>;
type Proof = <InputMmcs as Mmcs<V>>::MultiProof;

struct Fixture {
    input: InputMmcs,
    oracle: Stock,
    cap: TreeCap,
    data: CoefficientInputData,
    dimensions: Vec<Dimensions>,
    expected: Vec<Dense>,
}
