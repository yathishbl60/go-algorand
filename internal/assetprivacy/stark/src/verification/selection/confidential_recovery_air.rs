// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::{Domain, Layout};
use crate::recovery_group_air::{Operation, RecoveryGroupAir};

pub(super) fn air(
    domain: Domain,
    operation: Operation,
    authority: [u8; 32],
    participants: [[u8; 32]; 4],
) -> Result<RecoveryGroupAir, Error> {
    RecoveryGroupAir::confidential(
        domain.network,
        domain.asset,
        domain.policy,
        operation,
        authority,
        participants,
    )
    .ok_or(Error::Shape)
}

pub(super) fn fields(operation: Operation) -> (Layout, u64) {
    match operation {
        Operation::PartialPrivate => (Layout::ConfidentialPartialPrivate, 0),
        Operation::FullPrivate => (Layout::ConfidentialFullPrivate, 0),
        Operation::PartialPublic { credit } => (Layout::ConfidentialPartialPublic, credit),
        Operation::FullPublic { credit } => (Layout::ConfidentialFullPublic, credit),
    }
}
