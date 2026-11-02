// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

mod context;
mod corrupt;
mod files;
mod prove;
mod setup;

use algorand_asset_privacy_stark::{
    owner_registration_air::OwnerRegistrationAir, verification::selection::Prepared,
};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Owner = Prepared<OwnerRegistrationAir>;
const PROFILE: [u8; 64] = [31; 64];
// Deliberately public test data. This key MUST NEVER hold real assets.
const PUBLIC_TEST_SECRET: [u8; 32] = [51; 32];

pub fn run() -> Result<()> {
    let args: Vec<_> = std::env::args_os().skip(1).take(4).collect();
    eprintln!("TEST ONLY: fixtures use a known public spending secret; never fund this key");
    match args.as_slice() {
        [command, directory] if command == "setup" => setup::write(Path::new(directory)),
        [command, directory, context] if command == "prove" => {
            prove::write(Path::new(directory), Path::new(context))
        }
        _ => Err("usage: native_owner_fixture setup OUTPUTDIR | prove OUTPUTDIR CONTEXT".into()),
    }
}
