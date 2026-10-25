// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{DecodeLimits, Error};
use super::{BundleLimits, Expected, cursor::Cursor};

/// Verifies every required proof after complete framing/statement preflight.
/// Keys, kinds, roles and public values must come from the exact native plan.
pub fn verify_bundle(
    bytes: &[u8],
    slots: &[Expected<'_>],
    limits: BundleLimits,
    proof_limits: DecodeLimits,
) -> Result<(), Error> {
    preflight(bytes, slots, limits, proof_limits)?;
    let mut cursor = Cursor::start(bytes, slots, limits)?;
    for slot in slots {
        slot.target
            .verify(cursor.next(slot, limits)?, slot.values, proof_limits)?;
    }
    cursor.finish()
}

fn preflight(
    bytes: &[u8],
    slots: &[Expected<'_>],
    limits: BundleLimits,
    proof_limits: DecodeLimits,
) -> Result<(), Error> {
    let mut cursor = Cursor::start(bytes, slots, limits)?;
    for slot in slots {
        slot.target
            .header(cursor.next(slot, limits)?, proof_limits)?;
    }
    cursor.finish()
}
