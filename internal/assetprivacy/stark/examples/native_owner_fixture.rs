// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Test-only known-public-key fixtures, never a production wallet/prover command.
#[cfg(feature = "native-privacy")]
#[path = "support/native_owner/mod.rs"]
mod fixture;

#[cfg(feature = "native-privacy")]
fn main() {
    if let Err(error) = fixture::run() {
        eprintln!("native_owner_fixture: {error}");
        std::process::exit(1);
    }
}

#[cfg(not(feature = "native-privacy"))]
fn main() {
    eprintln!("TEST ONLY: native_owner_fixture requires --features native-privacy");
    std::process::exit(2);
}
