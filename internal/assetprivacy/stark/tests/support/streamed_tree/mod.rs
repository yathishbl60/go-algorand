// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod cases;
mod corruption;
mod fixture;
pub mod malformed;
mod oracle;

use algorand_asset_privacy_stark::compact::{CompactHash, Value32};
use p3_commit::Mmcs;
use p3_field::coset::TwoAdicMultiplicativeCoset;
use p3_matrix::{Dimensions, dense::RowMajorMatrix};
use p3_merkle_tree::MerkleTreeHidingMmcs;
use p3_symmetric::{CompressionFunctionFromHasher, SerializingHasher};
use rand::rngs::StdRng;

type V = Value32;
type Domain = TwoAdicMultiplicativeCoset<V>;
type Dense = RowMajorMatrix<V>;
type Leaf = SerializingHasher<CompactHash<1>>;
type Compress = CompressionFunctionFromHasher<CompactHash<2>, 2, 8>;
type Stock = MerkleTreeHidingMmcs<V, u64, Leaf, Compress, StdRng, 2, 8, 10>;
type StockData = <Stock as Mmcs<V>>::ProverData<Dense>;
type Cap = <Stock as Mmcs<V>>::Commitment;
type Opening = p3_commit::BatchOpening<V, Stock>;

struct Oracle {
    mmcs: Stock,
    cap: Cap,
    data: StockData,
    dimensions: Vec<Dimensions>,
}

// Public deterministic comparison seed; never seed a production hiding commitment this way.
const SALT_SEED: u64 = 61;
