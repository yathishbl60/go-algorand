// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Fixture, SECRET, fixture};
use algorand_asset_privacy_stark::{
    owner_registration_air::GroupWitness,
    verification::{ProveLimits, public_values_u16},
};

pub fn proof(fixture: &Fixture) -> (Vec<u8>, Vec<u16>) {
    let witness = fixture
        .prepared
        .air()
        .trace(SECRET, &fixture.statement)
        .expect("valid owner");
    canonical(fixture, witness)
}

fn canonical(fixture: &Fixture, witness: GroupWitness) -> (Vec<u8>, Vec<u16>) {
    let values = public_values_u16(&witness.public_values).expect("canonical public cells");
    let limits = ProveLimits::new(16 << 20, fixture::limits()).expect("bounded prover");
    let proof = fixture
        .prepared
        .prove(
            &fixture.registry,
            &fixture.setup,
            witness.trace,
            &values,
            limits,
        )
        .expect("artifact-bound proof with mandatory self-check");
    (proof, values)
}
