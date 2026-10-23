// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Degree-two SP800-38D multiplication and GHASH components, not full GCM.
//! Block bits follow transmitted byte order, most-significant bit first.

mod block;
mod chain;
mod chain_witness;
mod field;
mod multiply;
mod multiply_witness;

pub use block::{Block, from_aes_bits, to_aes_bits};
pub use chain::{Ghash, eval_ghash};
pub use multiply::{Gf128, eval_multiply};
