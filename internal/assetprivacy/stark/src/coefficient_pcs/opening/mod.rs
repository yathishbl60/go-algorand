// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

mod arithmetic;
mod capacity;
mod claims;
mod compression;
mod conversion;
mod denominators;
mod fri;
mod geometry;
mod reduction;
mod validation;

use super::{Challenger, CoefficientPcs, Domain, E, Error, ProverData, StockProof, V};
use p3_commit::{OpenedValues, OpeningRequest};
use p3_field::PrimeCharacteristicRing;
use p3_fri::{PcsProverTranscript, PcsShape};

type Request<'a> = OpeningRequest<'a, ProverData, E>;
type Transcript<'a> = PcsProverTranscript<'a, Challenger, V, E>;

pub(super) fn open(
    pcs: &CoefficientPcs,
    rounds: Vec<Request<'_>>,
    challenger: &mut Challenger,
    preprocessing: Option<usize>,
) -> Result<(OpenedValues<E>, StockProof), Error> {
    let max_log = validation::validate(pcs, &rounds, preprocessing)?;
    super::lease::consume(&rounds.iter().map(|r| r.prover_data).collect::<Vec<_>>())?;
    let mut values = claims::evaluate(&rounds)?;
    let proof = prove(pcs, &rounds, &values, challenger, max_log)?;
    let random = claims::split(&rounds, &mut values);
    Ok((values, (random, conversion::stock(proof))))
}

fn prove(
    pcs: &CoefficientPcs,
    rounds: &[Request<'_>],
    values: &OpenedValues<E>,
    challenger: &mut Challenger,
    max_log: usize,
) -> Result<fri::Proof, Error> {
    let shape = PcsShape::from_opened_values(&pcs.fri, values);
    let mut transcript = Transcript::new(challenger, shape);
    transcript.claimed_openings(values);
    let (alpha, witness) = transcript.batch_phase();
    let inputs = reduction::reduce(pcs, rounds, values, alpha)?;
    let boundary = (max_log, witness.unwrap_or(V::ZERO));
    let proof = fri::prove(pcs, rounds, inputs, &mut transcript, boundary);
    transcript.finish();
    proof
}

fn zeroes(length: usize) -> Result<Vec<E>, Error> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| Error::Allocation)?;
    values.resize(length, E::ZERO);
    Ok(values)
}
