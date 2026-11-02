// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Optional candidate Fp31/D10 arithmetic and a typed OS-seeded prover factory.
//! `prover_config()` returns a fresh configuration for each complete proof.
//! Fixed defaults are 192 queries, blowup 8, maximum FRI arity 8, final polynomial
//! length 1, ten random codewords, ten field-element leaf salts and zero PoW.
//! These are candidate parameters, without an established quantum soundness bound.
//! No native codec, verifier, accepted artifact or consensus profile is selected.
//! Existing root Value, Challenge, Config and prover_config retain their profile.
//! Complete wire/memory limits and application-relation security remain separate.

#[path = "compact_backend/mod.rs"]
mod backend;

pub use algorand_asset_privacy_field31::{Challenge32, Value32, has_full_extension_degree};
pub use backend::{
    Config, LOG_BLOWUP, LOG_FINAL_POLY_LEN, MAX_LOG_ARITY, NUM_QUERIES, RANDOM_CODEWORDS,
    SALT_ELEMS, prover_config,
};
