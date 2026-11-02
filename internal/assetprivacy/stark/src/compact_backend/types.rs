// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{SALT_ELEMS, hash::CompactHash};
use crate::compact::{Challenge32, Value32};
use p3_challenger::{HashChallenger, SerializingChallenger32};
use p3_commit::ExtensionMmcs;
use p3_dft::Radix2DitParallel;
use p3_fri::HidingFriPcs;
use p3_merkle_tree::MerkleTreeHidingMmcs;
use p3_symmetric::{CompressionFunctionFromHasher, SerializingHasher};
use p3_uni_stark::StarkConfig;
use rand::rngs::StdRng;

pub(super) type LeafHash = SerializingHasher<CompactHash<1>>;
pub(super) type Compress = CompressionFunctionFromHasher<CompactHash<2>, 2, 8>;
pub(super) type ValueMmcs =
    MerkleTreeHidingMmcs<Value32, u64, LeafHash, Compress, StdRng, 2, 8, SALT_ELEMS>;
pub(super) type ChallengeMmcs = ExtensionMmcs<Value32, Challenge32, ValueMmcs>;
pub(super) type Pcs =
    HidingFriPcs<Value32, Radix2DitParallel<Value32>, ValueMmcs, ChallengeMmcs, StdRng>;
pub(super) type Challenger =
    SerializingChallenger32<Value32, HashChallenger<u8, CompactHash<3>, 64>>;
pub type Config = StarkConfig<Pcs, Challenge32, Challenger>;
