// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod fixture;
mod generate;
pub mod proof;
pub mod registry;

use algorand_asset_privacy_stark::{
    owner_registration_air::{OwnerRegistrationAir, PublicStatement},
    verification::selection::{Pin, Prepared, PublicSetup, Registry},
};

pub struct Fixture {
    prepared: Prepared<OwnerRegistrationAir>,
    setup: PublicSetup,
    encoded: Vec<u8>,
    pin: Pin,
    registry: Registry,
    statement: PublicStatement,
}

const OWNER_RECORD: usize = 134 + 18 * 82;
const SECRET: [u8; 32] = [9; 32];
