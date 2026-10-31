// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::confidential_air::{shield_air, spend_air};
use super::{Domain, Layout, Prepared};
use crate::{
    amounts::PublicValue,
    shield_group_air::ShieldGroupAir,
    spend_group_air::{Operation, SpendGroupAir},
};

impl Prepared<SpendGroupAir> {
    pub fn confidential_transfer(
        domain: Domain,
        value: PublicValue,
        participants: [[u8; 32]; 4],
    ) -> Result<Self, Error> {
        let air = spend_air(domain, Operation::Transfer(value), participants)?;
        let mut args = domain.bytes().to_vec();
        args.extend_from_slice(&value.debit.to_be_bytes());
        args.extend_from_slice(&value.credit.to_be_bytes());
        Self::new(
            air,
            Layout::ConfidentialTransfer,
            &arguments(args, participants),
        )
    }
    pub fn confidential_unshield(
        domain: Domain,
        credit: u64,
        participants: [[u8; 32]; 4],
    ) -> Result<Self, Error> {
        let air = spend_air(domain, Operation::Unshield { credit }, participants)?;
        let mut args = domain.bytes().to_vec();
        args.extend_from_slice(&credit.to_be_bytes());
        Self::new(
            air,
            Layout::ConfidentialUnshield,
            &arguments(args, participants),
        )
    }
}

impl Prepared<ShieldGroupAir> {
    pub fn confidential_shield(
        domain: Domain,
        debit: u64,
        participants: [[u8; 32]; 4],
    ) -> Result<Self, Error> {
        let air = shield_air(domain, debit, participants)?;
        let mut args = domain.bytes().to_vec();
        args.extend_from_slice(&debit.to_be_bytes());
        Self::new(
            air,
            Layout::ConfidentialShield,
            &arguments(args, participants),
        )
    }
}

fn arguments(mut args: Vec<u8>, participants: [[u8; 32]; 4]) -> Vec<u8> {
    for account in participants {
        args.extend_from_slice(&account);
    }
    args
}
