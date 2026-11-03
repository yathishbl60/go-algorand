// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

mod authenticate;
mod claims;

use super::{Argument, Challenger, Claims, E, Fixture, Folding, configuration};
use p3_challenger::FieldChallenger;
use p3_fri::{PcsShape, PcsVerifierTranscript, TwoAdicFriFolding, verifier::verify_fri};
use std::marker::PhantomData;

pub fn accepts(f: &Fixture, argument: &Argument, context: u64) -> bool {
    let (authenticated, folded) = outcome(f, argument, context);
    authenticated && folded
}

pub fn outcome(f: &Fixture, argument: &Argument, context: u64) -> (bool, bool) {
    let mut challenger = configuration::challenger(&f.cap, context);
    let point = challenger.sample_algebra_element::<E>();
    let claims = claims::expected(f, argument, point);
    let shape = PcsShape::from_claims(&f.params, &claims);
    let mut transcript = PcsVerifierTranscript::new(&mut challenger, shape);
    transcript
        .claimed_openings(&claims)
        .expect("fixed claim dimensions");
    let alpha = transcript.batch_phase(None).expect("zero proof-of-work");
    let accepted =
        transcript.delegate(|challenger| checked_fri(f, argument, &claims, alpha, challenger));
    transcript.finish();
    accepted
}

fn checked_fri(
    f: &Fixture,
    a: &Argument,
    claims: &Claims,
    alpha: E,
    challenger: &mut Challenger,
) -> (bool, bool) {
    let mut replay = challenger.clone();
    let indices = authenticate::indices(f, &a.proof, &mut replay);
    let authenticated = authenticate::input(f, &a.proof, &indices);
    let folded = stock_fri(f, a, claims, alpha, challenger);
    (authenticated, folded)
}

fn stock_fri(
    f: &Fixture,
    a: &Argument,
    claims: &Claims,
    alpha: E,
    challenger: &mut Challenger,
) -> bool {
    let folding: Folding = TwoAdicFriFolding(PhantomData);
    verify_fri(
        &folding, &f.params, &a.proof, challenger, claims, &f.input, alpha,
    )
    .is_ok()
}
