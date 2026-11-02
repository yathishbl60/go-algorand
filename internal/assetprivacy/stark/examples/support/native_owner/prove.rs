// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Owner, PUBLIC_TEST_SECRET, Result, context::Context, corrupt, files, setup};
use algorand_asset_privacy_stark::{
    owner_registration_air::GroupWitness,
    verification::selection::{PublicSetup, Registry},
    verification::{DecodeLimits, ProveLimits, public_values_u16},
};
use std::path::Path;

pub fn write(directory: &Path, context_file: &Path) -> Result<()> {
    let context = Context::read(context_file)?;
    let registry = setup::load(directory)?;
    let prepared = Owner::owner_registration(context.domain)?;
    let public_setup = prepared.setup_public()?;
    registry.check_setup(&public_setup)?;
    let statement = context.statement();
    let witness = prepared
        .air()
        .trace(PUBLIC_TEST_SECRET, &statement)
        .ok_or("test owner witness construction failed")?;
    let (proof, bad) = canonical(&prepared, &registry, &public_setup, witness)?;
    publish(directory, &proof, &bad, &statement.owner)
}

fn canonical(
    prepared: &Owner,
    registry: &Registry,
    setup: &PublicSetup,
    witness: GroupWitness,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let values = public_values_u16(&witness.public_values)?;
    let decode = DecodeLimits::new(4_900_000, 64 << 20)?;
    let limits = ProveLimits::new(16 << 20, decode)?;
    let proof = prepared.prove(registry, setup, witness.trace, &values, limits)?;
    let bad = corrupt::proof(prepared, registry, &proof, &values)?;
    Ok((proof, bad))
}

fn publish(directory: &Path, proof: &[u8], bad: &[u8], owner: &[u8; 64]) -> Result<()> {
    files::write(&directory.join("proof.bin"), proof)?;
    files::write(&directory.join("bad-proof.bin"), bad)?;
    files::write(&directory.join("owner.bin"), owner)?;
    println!(
        "test_owner_proof_complete canonical_proof_bytes={}",
        proof.len()
    );
    Ok(())
}
