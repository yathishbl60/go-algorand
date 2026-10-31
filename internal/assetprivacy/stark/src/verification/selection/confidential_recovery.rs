// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::{Domain, Prepared};
use crate::recovery_group_air::{Operation, RecoveryGroupAir};

impl Prepared<RecoveryGroupAir> {
    pub fn confidential_recovery(
        domain: Domain,
        operation: Operation,
        authority: [u8; 32],
        participants: [[u8; 32]; 4],
    ) -> Result<Self, Error> {
        let air =
            super::confidential_recovery_air::air(domain, operation, authority, participants)?;
        let (layout, credit) = super::confidential_recovery_air::fields(operation);
        let mut args = domain.bytes().to_vec();
        args.extend_from_slice(&credit.to_be_bytes());
        args.extend_from_slice(&authority);
        for account in participants {
            args.extend_from_slice(&account);
        }
        Self::new(air, layout, &args)
    }
}
