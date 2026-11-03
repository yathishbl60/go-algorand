// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    Argument, Challenger, E, Fixture, Folding, LOG_ROWS, Proof, V, configuration, reduction,
};
use p3_challenger::FieldChallenger;
use p3_commit::{OpenedValues, OpeningRequest};
use p3_field::PrimeCharacteristicRing;
use p3_fri::{PcsProverTranscript, PcsShape, TwoAdicFriFolding, prover::prove_fri};
use std::marker::PhantomData;

type Driver<'a> = PcsProverTranscript<'a, Challenger, V, E>;
struct Reduction {
    point: E,
    word: Vec<E>,
    witness: V,
}

pub fn prove(fixture: &Fixture, context: u64) -> Argument {
    let mut challenger = configuration::challenger(&fixture.cap, context);
    let point = challenger.sample_algebra_element::<E>();
    assert!(algorand_asset_privacy_stark::compact::has_full_extension_degree(point));
    let values = reduction::openings(fixture, point);
    let proof = bind_and_prove(fixture, &mut challenger, point, &values);
    Argument { proof, values }
}

fn bind_and_prove(f: &Fixture, challenger: &mut Challenger, point: E, values: &[E]) -> Proof {
    let opened: OpenedValues<E> = vec![vec![vec![values.to_vec()]]];
    let shape = PcsShape::from_opened_values(&f.params, &opened);
    let mut transcript = Driver::new(challenger, shape);
    transcript.claimed_openings(&opened);
    let (alpha, witness) = transcript.batch_phase();
    let reduction = Reduction {
        point,
        word: reduction::word(f, point, alpha, values),
        witness: witness.unwrap_or(V::ZERO),
    };
    let proof = fri(&mut transcript, f, reduction);
    transcript.finish();
    proof
}

fn fri(transcript: &mut Driver<'_>, f: &Fixture, reduction: Reduction) -> Proof {
    let requests = requests(f, reduction.point);
    let folding: Folding = TwoAdicFriFolding(PhantomData);
    transcript
        .delegate(|challenger| {
            prove_fri(
                &folding,
                &f.params,
                vec![reduction.word],
                challenger,
                LOG_ROWS,
                &requests,
                &f.input,
                reduction.witness,
            )
        })
        .expect("real coefficient-backed FRI proving")
}

fn requests(f: &Fixture, point: E) -> [OpeningRequest<'_, super::CoefficientInputData, E>; 1] {
    [OpeningRequest {
        prover_data: &f.data,
        points: vec![vec![point]],
    }]
}
