// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::amounts::PublicValue;

/// Native verification selects the exact operation and public credit from bodies.
#[derive(Clone, Copy)]
pub enum Operation {
    PartialPrivate,
    FullPrivate,
    PartialPublic { credit: u64 },
    FullPublic { credit: u64 },
}

impl Operation {
    pub(super) fn valid(self) -> bool {
        self.private() || self.credit() != 0
    }
    pub(super) fn private(self) -> bool {
        matches!(self, Self::PartialPrivate | Self::FullPrivate)
    }
    pub(super) fn partial(self) -> bool {
        matches!(self, Self::PartialPrivate | Self::PartialPublic { .. })
    }
    pub(super) fn credit(self) -> u64 {
        match self {
            Self::PartialPublic { credit } | Self::FullPublic { credit } => credit,
            _ => 0,
        }
    }
    pub(super) fn output_count(self) -> usize {
        usize::from(self.private()) + usize::from(self.partial())
    }
    pub(super) fn remainder_index(self) -> Option<usize> {
        self.partial().then_some(usize::from(self.private()))
    }
    pub(super) fn amount_slot(self, output: usize) -> usize {
        if self.private() && output == 0 { 2 } else { 3 }
    }
    pub(super) fn conversion(self) -> PublicValue {
        PublicValue {
            debit: 0,
            credit: self.credit(),
        }
    }
    pub(super) fn conversion_start(self) -> usize {
        64 + 32 * self.output_count()
    }
    pub(super) fn authority_start(self) -> usize {
        self.public_count() - 32
    }
    pub(super) fn public_count(self) -> usize {
        128 + 32 * self.output_count() + if self.private() { 0 } else { 4 }
    }
    pub(super) fn credit_values(self) -> Vec<u16> {
        if self.private() {
            return Vec::new();
        }
        (0..4)
            .map(|limb| (self.credit() >> (16 * limb)) as u16)
            .collect()
    }
}
