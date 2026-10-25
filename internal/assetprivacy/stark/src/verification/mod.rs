// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded candidate verification; trusted artifacts must come from local configuration.
//! This module does not install a consensus suite or authenticate ledger statements.
mod artifact;
mod decode;
mod encode;
mod error;
mod fri;
mod limits;
mod metadata;
mod openings;
mod reader;
mod scalars;
mod shape;
mod verifier;
mod write_fri;
mod write_openings;
mod write_scalars;
mod writer;

pub use artifact::Artifact;
pub use error::Error;
pub use limits::DecodeLimits;
pub use p3_uni_stark::{PreprocessedVerifierKey, Proof as BackendProof};
pub use verifier::Verifier;

use crate::types::{ChallengeMmcs, ValueMmcs};
use crate::{Challenge, Config, Value};
use p3_fri::{BatchMultiOpening, FriProof};

type Proof = p3_uni_stark::Proof<Config>;
type Cap = p3_uni_stark::Com<Config>;
type Paths = p3_merkle_tree::PrunedMerklePaths<u64, 8>;
type Salts = Vec<Vec<Vec<Value>>>;
type MultiProof = (Salts, Paths);
type InputOpening = BatchMultiOpening<Value, ValueMmcs>;
type Fri = FriProof<Challenge, ChallengeMmcs, Value, Vec<InputOpening>>;
