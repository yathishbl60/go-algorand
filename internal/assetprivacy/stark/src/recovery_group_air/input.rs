// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::recovery_note_air::RecoveryInput;

/// One genuine recovery input plus exactly the operation's actual new outputs.
pub struct GroupInput<'a> {
    pub input: RecoveryInput<'a>,
    pub outputs: &'a [[u8; 512]],
}

pub(super) fn amount(opening: &[u8; 512]) -> u64 {
    u64::from_be_bytes(opening[169..177].try_into().expect("canonical note value"))
}
