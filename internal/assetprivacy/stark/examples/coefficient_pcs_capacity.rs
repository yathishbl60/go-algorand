// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Explicit heavy PCS capacity run over a synthetic public matrix.
#[cfg(feature = "native-privacy-compact")]
#[path = "support/coefficient_pcs_capacity/mod.rs"]
mod capacity;

#[cfg(feature = "native-privacy-compact")]
fn main() {
    capacity::run();
}

#[cfg(not(feature = "native-privacy-compact"))]
fn main() {
    eprintln!("coefficient_pcs_capacity requires --features native-privacy-compact");
    std::process::exit(2);
}
