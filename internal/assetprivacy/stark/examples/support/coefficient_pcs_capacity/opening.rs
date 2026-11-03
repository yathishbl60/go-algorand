// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Cap, Challenger, Claims, Config, Domain, E, Proof, Statement, V, compact, pcs};
use p3_challenger::{CanObserve, FieldChallenger};
use p3_commit::{CommitmentOpening, MatrixOpening, OpeningRequest, Pcs, PointOpening};
use p3_field::PrimeCharacteristicRing;
use p3_uni_stark::StarkGenericConfig;

pub(super) fn prove(
    config: &Config,
    cap: Cap,
    domain: Domain,
    data: &pcs::ProverData,
) -> (Statement, Claims, Proof) {
    let mut challenger = transcript(config, &cap);
    let z: E = challenger.sample_algebra_element();
    let points = vec![z, z * V::from_u8(7)];
    let requests = vec![OpeningRequest::from((data, vec![points.clone()]))];
    let pcs = config.pcs();
    let (values, proof) = pcs.open(requests, &mut challenger).expect("bounded open");
    report(&proof);
    let statement = Statement {
        cap,
        domain,
        points,
    };
    (statement, values, proof)
}

fn transcript(config: &Config, cap: &Cap) -> Challenger {
    let mut challenger = config.initialise_challenger();
    challenger.observe(V::from_usize(0x434150));
    challenger.observe(cap.clone());
    challenger
}

pub(super) fn verify(statement: Statement, values: Claims, proof: &Proof) {
    let stock = compact::prover_config().expect("OS entropy");
    let mut challenger = stock.initialise_challenger();
    challenger.observe(V::from_usize(0x434150));
    challenger.observe(statement.cap.clone());
    assert_eq!(
        statement.points[0],
        challenger.sample_algebra_element::<E>()
    );
    let claim = claim(statement, values);
    stock
        .pcs()
        .verify(vec![claim], proof, &mut challenger)
        .expect("stock verifier");
}

fn claim(statement: Statement, mut values: Claims) -> CommitmentOpening<E, Cap, Domain> {
    let values = values.remove(0).remove(0);
    let points = statement
        .points
        .into_iter()
        .zip(values)
        .map(|(point, values)| PointOpening { point, values })
        .collect();
    CommitmentOpening {
        commitment: statement.cap,
        matrices: vec![MatrixOpening {
            domain: statement.domain,
            points,
        }],
    }
}

fn report(proof: &Proof) {
    let input = &proof.1.input_openings[0];
    println!(
        "stage=full_pcs_open_ready disclosed_rows={} commitment_boundaries={}",
        input.opened_values.len(),
        input.opening_proof.1.sibling_hashes.len()
    );
}
