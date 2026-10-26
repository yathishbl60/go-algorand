// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::{Entry, Prepared};
use crate::Config;
use p3_uni_stark::{PreprocessedProverData, QuotientAir, setup_preprocessed};
use std::panic::{AssertUnwindSafe, catch_unwind};

/// Public-only reusable prover data; private masks/salts still need fresh prover_config.
pub struct PublicSetup {
    pub data: PreprocessedProverData<Config>,
    pub(super) entry: Entry,
}

impl PublicSetup {
    pub fn entry(&self) -> Entry {
        self.entry.clone()
    }
}

impl<A: QuotientAir<Config>> Prepared<A> {
    /// Expensive offline public setup, never called by native verification or build checks.
    pub fn setup_public(&self) -> Result<PublicSetup, Error> {
        let result = catch_unwind(AssertUnwindSafe(|| {
            setup_preprocessed(
                &crate::config::public_preprocessing_config(),
                &self.air,
                self.layout.rows().ilog2() as usize,
            )
        }))
        .map_err(|_| Error::BackendPanic)?
        .map_err(|_| Error::Artifact)?;
        let (data, key) = result.ok_or(Error::Artifact)?;
        Ok(PublicSetup {
            data,
            entry: Entry::new(self.layout, key)?,
        })
    }
}
