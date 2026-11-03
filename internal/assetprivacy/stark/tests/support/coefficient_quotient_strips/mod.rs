// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

mod current;
mod fixture;
mod guards;
pub mod notes;
#[path = "../owned_notes/fixture.rs"]
mod notes_fixture;
mod scheduler;

use algorand_asset_privacy_stark::{
    coefficient_pcs, compact, compact_owned_notes_air::OwnedNotesAir,
};

type V = compact::Value32;
type E = compact::Challenge32;
type Air = OwnedNotesAir<V>;
type Config = coefficient_pcs::Config;
type Domain = p3_field::coset::TwoAdicMultiplicativeCoset<V>;
