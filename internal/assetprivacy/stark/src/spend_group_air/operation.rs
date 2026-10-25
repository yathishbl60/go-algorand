// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{TRANSFER_PUBLIC_VALUES, UNSHIELD_PUBLIC_VALUES};
use crate::amounts::PublicValue;

/// The native verifier selects the variant and amounts from finalized bodies.
#[derive(Clone, Copy)]
pub enum Operation {
    Transfer(PublicValue),
    Unshield { credit: u64 },
}

impl Operation {
    pub(super) fn output_count(self) -> usize {
        match self {
            Self::Transfer(_) => 2,
            Self::Unshield { .. } => 0,
        }
    }
    pub(super) fn public_count(self) -> usize {
        match self {
            Self::Transfer(_) => TRANSFER_PUBLIC_VALUES,
            Self::Unshield { .. } => UNSHIELD_PUBLIC_VALUES,
        }
    }
    pub(super) fn conversion(self) -> PublicValue {
        match self {
            Self::Transfer(value) => value,
            Self::Unshield { credit } => PublicValue { debit: 0, credit },
        }
    }
    pub(super) fn conversion_values(self) -> Vec<u16> {
        let conversion = self.conversion();
        let amounts = match self {
            Self::Transfer(_) => vec![conversion.debit, conversion.credit],
            Self::Unshield { credit } => vec![credit],
        };
        amounts
            .into_iter()
            .flat_map(|amount| (0..4).map(move |limb| (amount >> (16 * limb)) as u16))
            .collect()
    }
    pub(super) fn conversion_start(self) -> usize {
        128 + self.output_count() * 32
    }
    pub(super) fn eligibility_start(self) -> usize {
        self.public_count() - 32
    }
}
