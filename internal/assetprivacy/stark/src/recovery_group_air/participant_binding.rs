// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{FLAGS, RecoveryGroupAir, WORK};
use crate::{Value, public_participants as parties};
use p3_air::{AirBuilder, WindowAccess};

impl RecoveryGroupAir {
    pub(super) fn eval_participants<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let Some(participants) = &self.participants else {
            return;
        };
        parties::eval_public(builder, participants, self.operation.public_count());
        for stage in 0..=self.operation.output_count() {
            let main = builder.main();
            let bits = self
                .participant_bits(main.current_slice(), stage)
                .expect("account wires");
            let flag = builder.preprocessed().current_slice()[FLAGS + stage * 4];
            let index = if stage == 0 { 0 } else { stage + 1 };
            parties::eval_account(builder, flag, &bits, &participants[index]);
        }
    }

    fn participant_bits<T: Copy>(&self, row: &[T], stage: usize) -> Option<[T; 256]> {
        if stage == 0 {
            return crate::recovery_note_air::wires(
                &row[WORK..WORK + crate::recovery_note_air::WIDTH],
            )?
            .account_bits
            .try_into()
            .ok();
        }
        self.output
            .account_bits(&row[WORK..WORK + self.output.width()])
    }
}
