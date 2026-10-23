// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_challenger::{HashChallenger, SerializingChallenger64};
use p3_commit::ExtensionMmcs;
use p3_dft::Radix2DitParallel;
use p3_field::extension::BinomialExtensionField;
use p3_fri::HidingFriPcs;
use p3_goldilocks::Goldilocks;
use p3_merkle_tree::MerkleTreeHidingMmcs;
use p3_symmetric::{CompressionFunctionFromHasher, SerializingHasher};
use p3_uni_stark::StarkConfig;
use rand::rngs::StdRng;

use crate::hash::DomainHash;

pub type Value = Goldilocks;
pub type Challenge = BinomialExtensionField<Value, 5>;
pub(crate) type LeafHash = SerializingHasher<DomainHash<1>>;
pub(crate) type Compress = CompressionFunctionFromHasher<DomainHash<2>, 2, 8>;
pub(crate) type ValueMmcs = MerkleTreeHidingMmcs<Value, u64, LeafHash, Compress, StdRng, 2, 8, 5>;
pub(crate) type ChallengeMmcs = ExtensionMmcs<Value, Challenge, ValueMmcs>;
pub(crate) type Pcs =
    HidingFriPcs<Value, Radix2DitParallel<Value>, ValueMmcs, ChallengeMmcs, StdRng>;
pub(crate) type Challenger = SerializingChallenger64<Value, HashChallenger<u8, DomainHash<3>, 64>>;
pub type Config = StarkConfig<Pcs, Challenge, Challenger>;
