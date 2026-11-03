// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub(crate) use crate::compact::{Challenge32 as E, Value32 as V};
use crate::compact::{CompactHash, Config as StockConfig, SALT_ELEMS};
use p3_challenger::{HashChallenger, SerializingChallenger32};
use p3_commit::{ExtensionMmcs, Pcs};
use p3_field::coset::TwoAdicMultiplicativeCoset;
use p3_matrix::dense::RowMajorMatrix;
use p3_merkle_tree::MerkleTreeHidingMmcs;
use p3_symmetric::{CompressionFunctionFromHasher, SerializingHasher};
use p3_uni_stark::{StarkConfig, StarkGenericConfig};
use rand::rngs::StdRng;

pub(crate) type Domain = TwoAdicMultiplicativeCoset<V>;
pub(crate) type Dense = RowMajorMatrix<V>;
pub(crate) type StockPcs = <StockConfig as StarkGenericConfig>::Pcs;
pub(crate) type Challenger = SerializingChallenger32<V, HashChallenger<u8, CompactHash<3>, 64>>;
pub(crate) type StockInput = MerkleTreeHidingMmcs<
    V,
    u64,
    SerializingHasher<CompactHash<1>>,
    CompressionFunctionFromHasher<CompactHash<2>, 2, 8>,
    StdRng,
    2,
    8,
    SALT_ELEMS,
>;
pub(crate) type FriMmcs = ExtensionMmcs<V, E, StockInput>;
pub(crate) type StockProof = <StockPcs as Pcs<E, Challenger>>::Proof;
pub(crate) type VerifierError = <StockPcs as Pcs<E, Challenger>>::Error;
pub type Config = StarkConfig<super::CoefficientPcs, E, Challenger>;
