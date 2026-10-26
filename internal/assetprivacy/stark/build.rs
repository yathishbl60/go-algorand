// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(feature = "native-privacy")]
#[path = "build_support/mod.rs"]
mod support;

#[cfg(feature = "native-privacy")]
fn main() -> std::io::Result<()> {
    support::emit()
}

#[cfg(not(feature = "native-privacy"))]
fn main() {}
