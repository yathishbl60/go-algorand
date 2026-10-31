// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{GroupInput, Operation, RecoveryGroupAir};
use crate::{Value, public_participants as parties};

impl RecoveryGroupAir {
    pub fn confidential(
        network: [u8; 32],
        asset: u64,
        policy: [u8; 64],
        operation: Operation,
        authority: [u8; 32],
        participants: parties::Participants,
    ) -> Option<Self> {
        parties::validate(&participants, 1, operation.output_count())?;
        if let Some(index) = operation.remainder_index()
            && participants[index + 2] != participants[0]
        {
            return None;
        }
        let mut air = Self::new(network, asset, policy, operation, authority)?;
        air.participants = Some(participants);
        Some(air)
    }

    pub(super) fn public_count(&self) -> usize {
        self.operation.public_count()
            + usize::from(self.participants.is_some()) * parties::PUBLIC_VALUES
    }

    pub(super) fn append_participants(&self, values: &mut Vec<Value>) {
        if let Some(participants) = &self.participants {
            parties::append(values, participants);
        }
    }

    pub(super) fn check_participant_openings(&self, input: &GroupInput<'_>) -> Option<()> {
        if let Some(participants) = &self.participants {
            parties::check_opening(&participants[0], input.input.opening)?;
            for (index, note) in input.outputs.iter().enumerate() {
                parties::check_opening(&participants[index + 2], note)?;
            }
        }
        Some(())
    }
}
