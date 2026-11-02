// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Owner, PROFILE, Result, files};
use algorand_asset_privacy_stark::verification::{
    implementation_id,
    selection::{Domain, Pin, Registry, encode_registry, registry_digest},
};
use std::{fs, path::Path};

pub fn write(directory: &Path) -> Result<()> {
    let prepared = Owner::owner_registration(setup_domain())?;
    let setup = prepared.setup_public()?;
    let encoded = encode_registry(PROFILE, &[setup.entry()])?;
    fs::create_dir_all(directory)?;
    files::write(&directory.join("registry.napk"), &encoded)?;
    files::write(&directory.join("profile.bin"), &PROFILE)?;
    files::write(&directory.join("implementation.bin"), &implementation_id())?;
    files::write(&directory.join("digest.bin"), &registry_digest(&encoded)?)?;
    println!("test_owner_setup_complete registry_bytes={}", encoded.len());
    Ok(())
}

pub fn load(directory: &Path) -> Result<Registry> {
    if files::read::<64>(&directory.join("profile.bin"))? != PROFILE {
        return Err("unexpected test profile".into());
    }
    let reference = files::read::<64>(&directory.join("implementation.bin"))?;
    if reference != implementation_id() {
        return Err("fixture implementation differs from this binary".into());
    }
    let pin = Pin {
        profile: PROFILE,
        implementation: reference,
        digest: files::read::<64>(&directory.join("digest.bin"))?,
    };
    let encoded = files::read::<1774>(&directory.join("registry.napk"))?;
    Ok(Registry::load(&encoded, pin)?)
}

fn setup_domain() -> Domain {
    Domain {
        network: [3; 32],
        asset: 7,
        policy: [19; 64],
    }
}
