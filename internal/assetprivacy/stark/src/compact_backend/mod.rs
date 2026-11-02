// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

mod config;
mod hash;
mod types;

pub use config::prover_config;
pub use types::Config;

pub const NUM_QUERIES: usize = 192;
pub const LOG_BLOWUP: usize = 3;
pub const MAX_LOG_ARITY: usize = 3;
pub const LOG_FINAL_POLY_LEN: usize = 0;
pub const RANDOM_CODEWORDS: usize = 10;
pub const SALT_ELEMS: usize = 10;
