// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CoefficientPcs, Config, Error, FriMmcs, Limits, StockInput};
use crate::compact::{self, CompactHash};
use crate::compact_storage::InputMmcs;
use p3_fri::FriParameters;
use p3_symmetric::{CompressionFunctionFromHasher, SerializingHasher};
use p3_uni_stark::StarkGenericConfig;
use rand::{
    SeedableRng,
    rngs::{StdRng, SysRng},
};
use std::sync::Mutex;

pub fn prover_config(limits: Limits) -> Result<Config, Error> {
    let baseline = compact::prover_config().map_err(|_| Error::Entropy)?;
    Ok(Config::new(
        CoefficientPcs::new(limits)?,
        baseline.initialise_challenger(),
    ))
}

impl CoefficientPcs {
    pub fn new(limits: Limits) -> Result<Self, Error> {
        limits.validate()?;
        let baseline = compact::prover_config().map_err(|_| Error::Entropy)?;
        Ok(Self {
            input: InputMmcs::new().map_err(|_| Error::Entropy)?,
            stock: baseline.pcs().clone(),
            fri: parameters()?,
            masks: Mutex::new(entropy()?),
            limits,
        })
    }

    pub(crate) fn fork_masks(&self) -> Result<StdRng, Error> {
        let mut source = self.masks.lock().map_err(|_| Error::RandomState)?;
        Ok(StdRng::from_rng(&mut *source))
    }
}

impl Clone for CoefficientPcs {
    fn clone(&self) -> Self {
        Self {
            input: self.input.clone(),
            stock: self.stock.clone(),
            fri: self.fri.clone(),
            masks: Mutex::new(self.fork_masks().expect("private mask state unavailable")),
            limits: self.limits,
        }
    }
}

fn entropy() -> Result<StdRng, Error> {
    StdRng::try_from_rng(&mut SysRng).map_err(|_| Error::Entropy)
}

fn parameters() -> Result<FriParameters<FriMmcs>, Error> {
    Ok(FriParameters {
        log_blowup: compact::LOG_BLOWUP,
        log_final_poly_len: compact::LOG_FINAL_POLY_LEN,
        max_log_arity: compact::MAX_LOG_ARITY,
        num_queries: compact::NUM_QUERIES,
        batch_proof_of_work_bits: 0,
        commit_proof_of_work_bits: 0,
        query_proof_of_work_bits: 0,
        mmcs: FriMmcs::new(StockInput::new(
            SerializingHasher::new(CompactHash::<1>),
            CompressionFunctionFromHasher::new(CompactHash::<2>),
            0,
            entropy()?,
        )),
    })
}
