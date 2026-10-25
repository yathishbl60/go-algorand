// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Candidate hiding-STARK relations. No accepted native consensus verifier yet.
#![forbid(unsafe_code)]

#[cfg(feature = "native-privacy")]
mod amount_witness;
#[cfg(feature = "native-privacy")]
mod config;
#[cfg(feature = "native-privacy")]
mod crypto_modules;
#[cfg(feature = "native-privacy")]
mod hash;
#[cfg(feature = "native-privacy")]
mod mlkem_modules;
#[cfg(feature = "native-privacy")]
mod note_modules;
#[cfg(feature = "native-privacy")]
mod nullifier_v2;
#[cfg(feature = "native-privacy")]
mod parameters;
#[cfg(feature = "native-privacy")]
mod prefix_builder;
#[cfg(feature = "native-privacy")]
mod private_output_builder;
#[cfg(feature = "native-privacy")]
mod public_segment_builder;
#[cfg(feature = "native-privacy")]
mod segment_builder;
#[cfg(feature = "native-privacy")]
mod types;
#[cfg(feature = "native-privacy")]
pub mod verification;

#[cfg(feature = "native-privacy")]
pub use config::prover_config;
#[cfg(feature = "native-privacy")]
pub use crypto_modules::*;
#[cfg(feature = "native-privacy")]
pub use mlkem_modules::*;
#[cfg(feature = "native-privacy")]
pub use note_modules::*;
#[cfg(feature = "native-privacy")]
pub use parameters::{MIN_TRACE_ROWS, NUM_QUERIES, RANDOM_CODEWORDS};
#[cfg(feature = "native-privacy")]
pub use types::{Challenge, Config, Value};
