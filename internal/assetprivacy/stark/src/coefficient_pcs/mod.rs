// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Candidate coefficient-backed PCS. Native/default selection remains unchanged.
mod budget;
mod commit;
mod error;
mod evaluations;
mod factory;
mod import;
mod lease;
mod limits;
mod opening;
mod pcs;
mod preflight;
mod quotient;
mod randomization;
mod stark_pcs;
mod stark_verify;
mod state;
mod types;

use crate::compact_storage::InputMmcs;
pub use error::Error;
pub use factory::prover_config;
pub use limits::Limits;
use p3_fri::FriParameters;
use rand::rngs::StdRng;
pub use state::ProverData;
use std::sync::Mutex;
pub use types::Config;
use types::*;

/// Fixed compact parameters and OS-backed masks; no implicit native activation.
pub struct CoefficientPcs {
    pub(crate) input: InputMmcs,
    pub(crate) stock: StockPcs,
    pub(crate) fri: FriParameters<FriMmcs>,
    pub(crate) masks: Mutex<StdRng>,
    pub(crate) limits: Limits,
}
