// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::arithmetic::{accumulate, dot};
use super::{CoefficientPcs, Domain, E, Error, Request, compression, denominators, zeroes};
use p3_commit::OpenedValues;
use p3_field::PrimeCharacteristicRing;

struct Reduction {
    values: [Option<Vec<E>>; 33],
    offsets: [usize; 33],
    alpha: E,
}
struct MatrixJob<'a> {
    domain: Domain,
    points: &'a [E],
    claims: &'a [Vec<E>],
    width: usize,
}

pub(super) fn reduce(
    pcs: &CoefficientPcs,
    rounds: &[Request<'_>],
    claims: &OpenedValues<E>,
    alpha: E,
) -> Result<Vec<Vec<E>>, Error> {
    let mut state = Reduction::new(alpha);
    for (round, round_claims) in rounds.iter().zip(claims) {
        reduce_round(pcs, &mut state, round, round_claims)?;
    }
    Ok(state.values.into_iter().rev().flatten().collect())
}

impl Reduction {
    fn new(alpha: E) -> Self {
        Self {
            values: core::array::from_fn(|_| None),
            offsets: [0; 33],
            alpha,
        }
    }
}

fn reduce_round(
    pcs: &CoefficientPcs,
    state: &mut Reduction,
    round: &Request<'_>,
    claims: &[Vec<Vec<E>>],
) -> Result<(), Error> {
    let domain = round.prover_data.inner.lde_domain;
    for (index, source) in round.prover_data.inner.sources.iter().enumerate() {
        let offset = state.alpha.exp_u64(state.offsets[domain.log_size()] as u64);
        let compressed = compression::compress(pcs, source, domain, state.alpha, offset)?;
        let job = MatrixJob {
            domain,
            points: &round.points[index],
            claims: &claims[index],
            width: source.width(),
        };
        matrix(state, job, &compressed)?;
    }
    Ok(())
}

fn matrix(state: &mut Reduction, job: MatrixJob<'_>, compressed: &[E]) -> Result<(), Error> {
    let log = job.domain.log_size();
    if state.values[log].is_none() {
        state.values[log] = Some(zeroes(job.domain.size())?);
    }
    for index in 0..job.points.len() {
        point(state, &job, compressed, index)?;
    }
    Ok(())
}

fn point(
    state: &mut Reduction,
    job: &MatrixJob<'_>,
    compressed: &[E],
    index: usize,
) -> Result<(), Error> {
    let log = job.domain.log_size();
    let inverse = denominators::inverse(job.domain, job.points[index])?;
    let offset = state.alpha.exp_u64(state.offsets[log] as u64);
    let weighted = offset * dot(state.alpha, &job.claims[index]);
    let rescale = state.alpha.exp_u64((index * job.width) as u64);
    let output = state.values[log].as_mut().ok_or(Error::Shape)?;
    accumulate(output, compressed, &inverse, weighted, rescale);
    state.offsets[log] += job.width;
    Ok(())
}
