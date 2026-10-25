// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{COMMIT, FLAGS, INVERSE, SpendableNoteAir, WORK};
use crate::{Value, nullifier_v2};
use p3_air::{AirBuilder, WindowAccess};
use p3_keccak_air::NUM_KECCAK_COLS;

impl SpendableNoteAir {
    pub(super) fn eval_nullifier_commitment<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let gate = builder.preprocessed().current_slice()[FLAGS + 8];
        let start = WORK + NUM_KECCAK_COLS + nullifier_v2::COMMITMENT_BYTE * 8;
        nullifier_v2::bind_commitment(
            builder,
            gate.into(),
            &row[COMMIT..INVERSE],
            &row[start..start + 512],
        );
    }
}
