// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::{Domain, Layout, Prepared};
use crate::{
    amounts::PublicValue,
    recovery_group_air::{Operation as Recovery, RecoveryGroupAir},
    spend_group_air::{Operation as Spend, SpendGroupAir},
};

impl Prepared<SpendGroupAir> {
    pub fn transfer(domain: Domain, value: PublicValue) -> Result<Self, Error> {
        let air = SpendGroupAir::new(
            domain.network,
            domain.asset,
            domain.policy,
            Spend::Transfer(value),
        )
        .ok_or(Error::Shape)?;
        let mut args = domain.bytes().to_vec();
        args.extend_from_slice(&value.debit.to_be_bytes());
        args.extend_from_slice(&value.credit.to_be_bytes());
        Self::new(air, Layout::Transfer, &args)
    }

    pub fn unshield(domain: Domain, credit: u64) -> Result<Self, Error> {
        let air = SpendGroupAir::new(
            domain.network,
            domain.asset,
            domain.policy,
            Spend::Unshield { credit },
        )
        .ok_or(Error::Shape)?;
        let mut args = domain.bytes().to_vec();
        args.extend_from_slice(&credit.to_be_bytes());
        Self::new(air, Layout::Unshield, &args)
    }
}

impl Prepared<RecoveryGroupAir> {
    pub fn recovery(
        domain: Domain,
        operation: Recovery,
        authority: [u8; 32],
    ) -> Result<Self, Error> {
        let air = RecoveryGroupAir::new(
            domain.network,
            domain.asset,
            domain.policy,
            operation,
            authority,
        )
        .ok_or(Error::Shape)?;
        let (layout, credit) = recovery_fields(operation);
        let mut args = domain.bytes().to_vec();
        args.extend_from_slice(&credit.to_be_bytes());
        args.extend_from_slice(&authority);
        Self::new(air, layout, &args)
    }
}

fn recovery_fields(operation: Recovery) -> (Layout, u64) {
    match operation {
        Recovery::PartialPrivate => (Layout::PartialPrivate, 0),
        Recovery::FullPrivate => (Layout::FullPrivate, 0),
        Recovery::PartialPublic { credit } => (Layout::PartialPublic, credit),
        Recovery::FullPublic { credit } => (Layout::FullPublic, credit),
    }
}
