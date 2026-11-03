// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Stock dense MMCS with a typed, coefficient-backed equal-height fast path.
mod commit;
mod error;
mod factory;
mod gather;
mod limits;
mod open;
mod state;
mod traits;

use super::{CoefficientMatrix, MaskedTrace, StreamedTree, TreeCap};
use crate::compact::{CompactHash, SALT_ELEMS, Value32};
use algorand_asset_privacy_matrix_adapter::SafeMatrix;
use p3_commit::Mmcs;
use p3_merkle_tree::MerkleTreeHidingMmcs;
use p3_symmetric::{CompressionFunctionFromHasher, SerializingHasher};
use rand::rngs::StdRng;
use std::sync::{Arc, Mutex};

pub use error::InputError;
pub use limits::StreamedInputLimits;
pub use state::InputProverData;
use state::{Storage, Streamed};

type LeafHash = SerializingHasher<CompactHash<1>>;
type Compress = CompressionFunctionFromHasher<CompactHash<2>, 2, 8>;
type Stock = MerkleTreeHidingMmcs<Value32, u64, LeafHash, Compress, StdRng, 2, 8, SALT_ELEMS>;
type DenseData<M> = <Stock as Mmcs<Value32>>::ProverData<M>;
type Proof = <Stock as Mmcs<Value32>>::Proof;
type MultiProof = <Stock as Mmcs<Value32>>::MultiProof;
type MultiValues = Vec<Vec<Vec<Value32>>>;
pub type OwnedCoefficientMatrix = SafeMatrix<CoefficientMatrix<Arc<MaskedTrace<Value32>>>, Value32>;
pub type CoefficientInputData = InputProverData<OwnedCoefficientMatrix>;

/// Clones share monotonic private RNG streams; they never repeat a seeded stream.
/// No hiding marker is claimed until complete backend review and integration.
#[derive(Clone)]
pub struct InputMmcs {
    stock: Arc<Stock>,
    random: Arc<Mutex<StdRng>>,
}
