// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Error, ProveLimits};
use super::{Prepared, PublicSetup, Registry};
use crate::{Config, Value};
use p3_air::{Air, DebugConstraintBuilder};
use p3_matrix::dense::RowMajorMatrix;
use p3_uni_stark::{QuotientAir, VerifierConstraintFolder};

impl<A> Prepared<A>
where
    A: Clone
        + QuotientAir<Config>
        + for<'a> Air<DebugConstraintBuilder<'a, Value>>
        + for<'a> Air<VerifierConstraintFolder<'a, Config>>,
{
    /// Check pinned setup and nominal trace budget before invoking a private builder.
    /// The callback cannot supply a verifier, configuration, entropy or acceptance result.
    pub fn prove_with<F>(
        &self,
        registry: &Registry,
        setup: &PublicSetup,
        public_values: &[u16],
        limits: ProveLimits,
        build: F,
    ) -> Result<Vec<u8>, Error>
    where
        F: FnOnce(&A) -> Result<RowMajorMatrix<Value>, Error>,
    {
        self.check_proving_inputs(registry, setup, public_values)?;
        self.prover_requirements(limits)?;
        self.prove(registry, setup, build(&self.air)?, public_values, limits)
    }
}
