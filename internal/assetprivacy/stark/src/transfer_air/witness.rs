// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::membership_air::MembershipPath;

/// Secret input material; keep it local and never include it in node RPC data.
pub struct InputWitness<'a> {
    pub opening: &'a [u8; 512],
    pub secret: [u8; 32],
    pub path: &'a MembershipPath,
}

/// Exactly two actual member inputs and two canonical private output openings.
/// This does not define dummy notes, optional slots, or their consensus semantics.
pub struct TransferWitness<'a> {
    pub inputs: [InputWitness<'a>; 2],
    pub outputs: &'a [[u8; 512]; 2],
}

pub(super) fn amount(opening: &[u8; 512]) -> u64 {
    u64::from_be_bytes(opening[169..177].try_into().expect("uint64 value field"))
}
