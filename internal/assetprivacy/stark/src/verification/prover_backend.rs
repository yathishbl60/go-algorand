// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Error, Proof};
use crate::{Config, Value};
use p3_air::{Air, DebugConstraintBuilder};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;
use p3_uni_stark::{PreprocessedProverData, QuotientAir, prove_with_preprocessed};

pub(super) fn prove<A>(
    air: &A,
    setup: &PreprocessedProverData<Config>,
    trace: RowMajorMatrix<Value>,
    public_values: &[u16],
) -> Result<Proof, Error>
where
    A: QuotientAir<Config> + for<'a> Air<DebugConstraintBuilder<'a, Value>>,
{
    let config = crate::prover_config().map_err(|_| Error::EntropyUnavailable)?;
    let values: Vec<_> = public_values.iter().copied().map(Value::from_u16).collect();
    prove_with_preprocessed(&config, air, trace, &values, Some(setup))
        .map_err(|_| Error::ProvingFailure)
}
