// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Exact main/output proof composition; expected slots and keys are selected locally.
//! No native transaction authentication or state transition is performed here.
mod binding;
mod cursor;
mod encode;
mod expected;
mod layout;
mod limits;
mod main_slots;
mod output_slots;
mod target;
mod verify;

pub use encode::encode_bundle;
pub use expected::Expected;
pub use limits::BundleLimits;
pub use verify::verify_bundle;

pub(super) fn validate_expected(slots: &[Expected<'_>]) -> Result<(), super::Error> {
    layout::check(slots)
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum RecoveryKind {
    PartialPrivate = 4,
    FullPrivate = 5,
    PartialPublic = 6,
    FullPublic = 7,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum DisclosureRole {
    Auditor = 2,
    Controller = 3,
}

const MAIN_OUTPUT: u32 = u32::MAX;
const HEADER: usize = 7;
const ENTRY: usize = 10;
const MAX_SLOTS: usize = 7;
const MAGIC: [u8; 5] = *b"NAPB\x01";
