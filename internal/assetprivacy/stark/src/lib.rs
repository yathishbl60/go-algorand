// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Candidate hiding-STARK relations. No accepted native consensus verifier yet.
#![forbid(unsafe_code)]

#[cfg(feature = "native-privacy")]
pub mod aes256_air;
#[cfg(feature = "native-privacy")]
pub mod aes_air;
#[cfg(feature = "native-privacy")]
pub mod aes_gcm_vertical;
#[cfg(feature = "native-privacy")]
mod amount_witness;
#[cfg(feature = "native-privacy")]
pub mod amounts;
#[cfg(feature = "native-privacy")]
mod config;
#[cfg(feature = "native-privacy")]
pub mod gcm_air;
#[cfg(feature = "native-privacy")]
pub mod gcm_vertical;
#[cfg(feature = "native-privacy")]
pub mod ghash_air;
#[cfg(feature = "native-privacy")]
pub mod ghash_chain;
#[cfg(feature = "native-privacy")]
pub mod ghash_vertical;
#[cfg(feature = "native-privacy")]
mod hash;
#[cfg(feature = "native-privacy")]
pub mod hkdf_air;
#[cfg(feature = "native-privacy")]
pub mod hmac_air;
#[cfg(feature = "native-privacy")]
pub mod member_note_air;
#[cfg(feature = "native-privacy")]
pub mod membership_air;
#[cfg(feature = "native-privacy")]
pub mod mlkem_ciphertext_air;
#[cfg(feature = "native-privacy")]
pub mod mlkem_derivation_air;
#[cfg(feature = "native-privacy")]
pub mod mlkem_dot_air;
#[cfg(feature = "native-privacy")]
pub mod mlkem_encoding_air;
#[cfg(feature = "native-privacy")]
pub mod mlkem_field;
#[cfg(feature = "native-privacy")]
pub mod mlkem_host;
#[cfg(feature = "native-privacy")]
pub mod mlkem_matrix_air;
#[cfg(feature = "native-privacy")]
pub mod mlkem_noise_air;
#[cfg(feature = "native-privacy")]
pub mod mlkem_ntt_air;
#[cfg(feature = "native-privacy")]
pub mod mlkem_pke_air;
#[cfg(feature = "native-privacy")]
pub mod mlkem_product_air;
#[cfg(feature = "native-privacy")]
pub mod mlkem_sample_air;
#[cfg(feature = "native-privacy")]
pub mod note_air;
#[cfg(feature = "native-privacy")]
pub mod nullified_note_air;
#[cfg(feature = "native-privacy")]
pub mod owned_note_air;
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
pub mod sha3_air;
#[cfg(feature = "native-privacy")]
pub mod shake128_matrix_air;
#[cfg(feature = "native-privacy")]
pub mod shake_air;
#[cfg(feature = "native-privacy")]
pub mod shield_air;
#[cfg(feature = "native-privacy")]
pub mod transfer_air;
#[cfg(feature = "native-privacy")]
mod types;
#[cfg(feature = "native-privacy")]
pub mod unshield_air;

#[cfg(feature = "native-privacy")]
pub use config::prover_config;
#[cfg(feature = "native-privacy")]
pub use parameters::{MIN_TRACE_ROWS, NUM_QUERIES, RANDOM_CODEWORDS};
#[cfg(feature = "native-privacy")]
pub use types::{Challenge, Config, Value};
