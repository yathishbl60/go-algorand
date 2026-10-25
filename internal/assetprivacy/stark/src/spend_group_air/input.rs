// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::spendable_note_air::SpendInput;

/// Two real holder inputs and exactly two transfer outputs or no unshield outputs.
/// Secret openings, ownership material and paths must remain local to the prover.
pub struct GroupInput<'a> {
    pub inputs: [SpendInput<'a>; 2],
    pub outputs: &'a [[u8; 512]],
}

pub(super) fn amount(opening: &[u8; 512]) -> u64 {
    u64::from_be_bytes(opening[169..177].try_into().expect("canonical note value"))
}
