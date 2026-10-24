// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Deterministic ML-KEM-768 encryption witness generation, not a verifier.
mod derive;
mod encode;
mod key;
mod noise;
mod polynomial;
mod types;
mod witness;

pub use derive::encapsulate;
pub use types::{Encapsulation, EncryptionWitness, Polynomial};
