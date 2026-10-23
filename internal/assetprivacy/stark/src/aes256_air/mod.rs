// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Complete AES-256 block/key-schedule constraints; GCM and KEM remain separate.
//! State bytes use AES column-major ordering, and each byte's bits are little endian.

mod add_key;
mod byte;
mod key;
mod key_binding;
mod key_witness;
mod mix;
mod mix_witness;
mod round;
mod witness;
mod witness_round;

pub use key::{ExpandedKey, eval_key};
pub use mix::{MixColumn, eval_mix};
pub use round::{Aes256, MixRounds, States, eval_cipher};
