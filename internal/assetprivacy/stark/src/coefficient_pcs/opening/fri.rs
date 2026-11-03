// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::FriMmcs;
use super::{CoefficientPcs, E, Error, Request, Transcript, V};
use crate::compact_storage::{CoefficientInputData, InputMmcs};
use p3_commit::OpeningRequest;
use p3_fri::{BatchMultiOpening, FriProof, TwoAdicFriFolding, TwoAdicFriFoldingForMmcs};
use std::marker::PhantomData;

pub(super) type Proof = FriProof<E, FriMmcs, V, Vec<BatchMultiOpening<V, InputMmcs>>>;

pub(super) fn prove(
    pcs: &CoefficientPcs,
    rounds: &[Request<'_>],
    inputs: Vec<Vec<E>>,
    transcript: &mut Transcript<'_>,
    boundary: (usize, V),
) -> Result<Proof, Error> {
    let requests = requests(rounds);
    let (max_log, witness) = boundary;
    let folding: TwoAdicFriFoldingForMmcs<V, InputMmcs> = TwoAdicFriFolding(PhantomData);
    transcript.delegate(|challenger| {
        p3_fri::prover::prove_fri(
            &folding, &pcs.fri, inputs, challenger, max_log, &requests, &pcs.input, witness,
        )
        .map_err(Error::from)
    })
}

fn requests<'a>(rounds: &[Request<'a>]) -> Vec<OpeningRequest<'a, CoefficientInputData, E>> {
    rounds
        .iter()
        .map(|round| OpeningRequest {
            prover_data: &round.prover_data.inner.input,
            points: round.points.clone(),
        })
        .collect()
}
