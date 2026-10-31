// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{FLAGS, GroupInput, Operation, SpendGroupAir, WORK};
use crate::{Value, public_participants as parties};
use p3_air::{AirBuilder, WindowAccess};

impl SpendGroupAir {
    pub fn confidential(
        network: [u8; 32],
        asset: u64,
        policy: [u8; 64],
        operation: Operation,
        participants: parties::Participants,
    ) -> Option<Self> {
        parties::validate(&participants, 2, operation.output_count())?;
        let mut air = Self::new(network, asset, policy, operation)?;
        air.participants = Some(participants);
        Some(air)
    }

    pub(super) fn public_count(&self) -> usize {
        self.operation.public_count()
            + usize::from(self.participants.is_some()) * parties::PUBLIC_VALUES
    }

    pub(super) fn check_participant_openings(&self, input: &GroupInput<'_>) -> Option<()> {
        if let Some(participants) = &self.participants {
            for (index, note) in input.inputs.iter().enumerate() {
                parties::check_opening(&participants[index], note.opening)?;
            }
            for (index, note) in input.outputs.iter().enumerate() {
                parties::check_opening(&participants[index + 2], note)?;
            }
        }
        Some(())
    }

    pub(super) fn eval_participants<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let Some(participants) = &self.participants else {
            return;
        };
        parties::eval_public(builder, participants, self.operation.public_count());
        for (index, account) in participants
            .iter()
            .enumerate()
            .take(2 + self.operation.output_count())
        {
            let main = builder.main();
            let bits = self
                .participant_bits(main.current_slice(), index)
                .expect("account wires");
            let flag = builder.preprocessed().current_slice()[FLAGS + index * 4];
            parties::eval_account(builder, flag, &bits, account);
        }
    }

    fn participant_bits<T: Copy>(&self, row: &[T], index: usize) -> Option<[T; 256]> {
        if index < 2 {
            return crate::spendable_note_air::wires(&row[WORK..])?
                .account_bits
                .try_into()
                .ok();
        }
        self.output
            .account_bits(&row[WORK..WORK + self.output.width()])
    }
}
