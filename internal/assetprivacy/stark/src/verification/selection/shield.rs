// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::{Domain, Layout, Prepared};
use crate::shield_group_air::ShieldGroupAir;

impl Prepared<ShieldGroupAir> {
    pub fn shield(domain: Domain, debit: u64) -> Result<Self, Error> {
        let air = ShieldGroupAir::new(domain.network, domain.asset, domain.policy, debit)
            .ok_or(Error::Shape)?;
        let mut args = domain.bytes().to_vec();
        args.extend_from_slice(&debit.to_be_bytes());
        Self::new(air, Layout::Shield, &args)
    }
}
