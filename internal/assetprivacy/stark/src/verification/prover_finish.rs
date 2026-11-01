// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Error, ProveLimits, Verifier, prover_backend};
use crate::{Config, Value};
use p3_air::{Air, DebugConstraintBuilder};
use p3_matrix::dense::RowMajorMatrix;
use p3_uni_stark::{PreprocessedProverData, QuotientAir, VerifierConstraintFolder};
use std::panic::{AssertUnwindSafe, catch_unwind};

type Task<'a> = (RowMajorMatrix<Value>, &'a [u16], ProveLimits);

pub(super) fn finish<A>(
    verifier: &Verifier<A>,
    air: &A,
    setup: &PreprocessedProverData<Config>,
    task: Task<'_>,
) -> Result<Vec<u8>, Error>
where
    A: QuotientAir<Config>
        + for<'a> Air<DebugConstraintBuilder<'a, Value>>
        + for<'a> Air<VerifierConstraintFolder<'a, Config>>,
{
    catch_unwind(AssertUnwindSafe(|| {
        encode_verified(verifier, air, setup, task)
    }))
    .map_err(|_| Error::BackendPanic)?
}

fn encode_verified<A>(
    verifier: &Verifier<A>,
    air: &A,
    setup: &PreprocessedProverData<Config>,
    task: Task<'_>,
) -> Result<Vec<u8>, Error>
where
    A: QuotientAir<Config>
        + for<'a> Air<DebugConstraintBuilder<'a, Value>>
        + for<'a> Air<VerifierConstraintFolder<'a, Config>>,
{
    let (trace, public_values, limits) = task;
    let proof = prover_backend::prove(air, setup, trace, public_values)?;
    let encoded = verifier.encode(&proof, limits.proof)?;
    drop(proof);
    verifier.verify(&encoded, public_values, limits.proof)?;
    Ok(encoded)
}
