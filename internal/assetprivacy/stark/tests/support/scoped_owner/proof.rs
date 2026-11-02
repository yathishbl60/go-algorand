// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Fixture, OWNER_RECORD, fixture, generate};
use algorand_asset_privacy_stark::verification::selection::{Prepared, Registry};

pub fn round_trip(fixture: &Fixture) {
    let (proof, values) = generate::proof(fixture);
    assert!(proof.len() < 4_900_000);
    let verifier = fixture
        .registry
        .bind(fixture.prepared.clone())
        .expect("exact kind binding");
    assert!(verifier.verify(&proof, &values, fixture::limits()).is_ok());
    eprintln!("scoped_owner_canonical_proof_bytes={}", proof.len());
    changed_values(fixture, &proof, &values);
    changed_proof(fixture, &proof, &values);
    changed_artifacts(fixture, &proof, &values);
}

fn changed_values(fixture: &Fixture, proof: &[u8], values: &[u16]) {
    let verifier = fixture
        .registry
        .bind(fixture.prepared.clone())
        .expect("owner verifier");
    for index in [0, 32] {
        let mut changed = values.to_vec();
        changed[index] ^= 1;
        assert!(verifier.verify(proof, &changed, fixture::limits()).is_err());
    }
}

fn changed_proof(fixture: &Fixture, proof: &[u8], values: &[u16]) {
    let verifier = fixture
        .registry
        .bind(fixture.prepared.clone())
        .expect("owner verifier");
    for index in [8, 100] {
        let mut changed = proof.to_vec();
        changed[index] ^= 1;
        assert!(
            verifier
                .verify(&changed, values, fixture::limits())
                .is_err()
        );
    }
    assert!(
        verifier
            .verify(&proof[..proof.len() - 1], values, fixture::limits())
            .is_err()
    );
}

fn changed_artifacts(fixture: &Fixture, proof: &[u8], values: &[u16]) {
    let mut domain = fixture::domain();
    domain.asset += 1;
    let prepared = Prepared::owner_registration(domain).expect("different typed relation");
    let verifier = fixture
        .registry
        .bind(prepared)
        .expect("same schedule, different relation");
    assert!(verifier.verify(proof, values, fixture::limits()).is_err());
    let mut changed = fixture.encoded.clone();
    changed[OWNER_RECORD + 18] ^= 1;
    let registry = Registry::load(&changed, fixture::pin(&changed)).expect("different pinned root");
    assert!(registry.check_setup(&fixture.setup).is_err());
    let verifier = registry
        .bind(fixture.prepared.clone())
        .expect("different trusted artifact");
    assert!(verifier.verify(proof, values, fixture::limits()).is_err());
}
