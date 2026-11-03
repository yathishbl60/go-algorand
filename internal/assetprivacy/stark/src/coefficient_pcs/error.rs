// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::compact_storage::InputError;

#[derive(Debug)]
pub enum Error {
    Shape,
    Limits,
    Allocation,
    Entropy,
    RandomState,
    OpeningUsed,
    HidingBudget,
    Unsupported,
    Input(InputError),
    Fri(p3_fri::FriProverError),
    Stock(p3_fri::HidingFriProverError),
}

impl core::fmt::Display for Error {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "coefficient PCS preparation failed: {self:?}")
    }
}

impl std::error::Error for Error {}

impl From<InputError> for Error {
    fn from(value: InputError) -> Self {
        Self::Input(value)
    }
}

impl From<p3_fri::FriProverError> for Error {
    fn from(value: p3_fri::FriProverError) -> Self {
        Self::Fri(value)
    }
}

impl From<p3_fri::HidingFriProverError> for Error {
    fn from(value: p3_fri::HidingFriProverError) -> Self {
        Self::Stock(value)
    }
}
