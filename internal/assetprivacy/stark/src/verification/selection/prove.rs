// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Error, ProveLimits, prover_finish};
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
    /// Prove locally with fresh entropy, canonical encoding and mandatory self-checking.
    /// The consumed trace is secret; callers must not log or persist its raw contents.
    pub fn prove(
        &self,
        registry: &Registry,
        setup: &PublicSetup,
        trace: RowMajorMatrix<Value>,
        public_values: &[u16],
        limits: ProveLimits,
    ) -> Result<Vec<u8>, Error> {
        self.check_proving_inputs(registry, setup, public_values)?;
        self.prover_requirements(limits)?.check_trace(&trace)?;
        limits.check_trace_capacity(&trace)?;
        let verifier = registry.bind(self.clone())?;
        let task = (trace, public_values, limits);
        prover_finish::finish(&verifier, &self.air, &setup.data, task)
    }
}
