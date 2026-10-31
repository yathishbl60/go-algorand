// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{NOTE_WIDTH, PUBLIC_VALUES, ShieldGroupAir, WORK};
use crate::{Value, public_participants as parties};
use p3_air::{AirBuilder, WindowAccess};

impl ShieldGroupAir {
    pub fn confidential(
        network: [u8; 32],
        asset: u64,
        policy: [u8; 64],
        debit: u64,
        participants: parties::Participants,
    ) -> Option<Self> {
        parties::validate(&participants, 0, 2)?;
        let mut air = Self::new(network, asset, policy, debit)?;
        air.participants = Some(participants);
        Some(air)
    }

    pub(super) fn public_count(&self) -> usize {
        PUBLIC_VALUES + usize::from(self.participants.is_some()) * parties::PUBLIC_VALUES
    }

    pub(super) fn check_participant_openings(&self, openings: &[[u8; 512]; 2]) -> Option<()> {
        if let Some(participants) = &self.participants {
            for (index, note) in openings.iter().enumerate() {
                parties::check_opening(&participants[index + 2], note)?;
            }
        }
        Some(())
    }

    pub(super) fn eval_participants<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let Some(participants) = &self.participants else {
            return;
        };
        parties::eval_public(builder, participants, PUBLIC_VALUES);
        for (index, account) in participants[2..].iter().enumerate() {
            let main = builder.main();
            let bits = self
                .output
                .account_bits(&main.current_slice()[WORK..WORK + NOTE_WIDTH])
                .expect("account wires");
            let flag = builder.preprocessed().current_slice()[index * 4];
            parties::eval_account(builder, flag, &bits, account);
        }
    }
}
