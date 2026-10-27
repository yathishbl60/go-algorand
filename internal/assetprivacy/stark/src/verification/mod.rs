// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bounded candidate verification; trusted artifacts must come from local configuration.
//! This module does not install a consensus suite or authenticate ledger statements.
mod access;
mod allocation;
mod artifact;
pub mod bundle;
mod decode;
mod encode;
mod error;
mod fri;
mod implementation;
mod limits;
mod metadata;
pub mod native;
mod openings;
mod reader;
mod scalars;
pub mod selection;
mod shape;
mod verifier;
mod write_fri;
mod write_openings;
mod write_scalars;
mod writer;

use artifact::Artifact;
pub use error::Error;
pub use implementation::implementation_id;
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
