// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Degree-two AES byte arithmetic components, not a complete encryption relation.
//! Bits are little endian. Callers must bind columns to their actual AES state.

mod affine;
mod byte;
mod field;
mod multiply;
mod multiply_witness;
mod sbox;
mod sbox_witness;

pub use byte::{Byte, eval_byte, eval_xtime};
pub use multiply::{GfMul, eval_multiply};
pub use sbox::{SBox, eval_sbox};
