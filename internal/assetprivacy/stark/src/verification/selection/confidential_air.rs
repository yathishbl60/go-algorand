// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::Domain;
use crate::{
    shield_group_air::ShieldGroupAir,
    spend_group_air::{Operation, SpendGroupAir},
};

pub(super) fn spend_air(
    domain: Domain,
    operation: Operation,
    participants: [[u8; 32]; 4],
) -> Result<SpendGroupAir, Error> {
    SpendGroupAir::confidential(
        domain.network,
        domain.asset,
        domain.policy,
        operation,
        participants,
    )
    .ok_or(Error::Shape)
}

pub(super) fn shield_air(
    domain: Domain,
    debit: u64,
    participants: [[u8; 32]; 4],
) -> Result<ShieldGroupAir, Error> {
    ShieldGroupAir::confidential(
        domain.network,
        domain.asset,
        domain.policy,
        debit,
        participants,
    )
    .ok_or(Error::Shape)
}
