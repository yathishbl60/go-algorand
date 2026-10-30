// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::{Domain, Layout, Prepared};
use crate::owner_registration_air::OwnerRegistrationAir;

impl Prepared<OwnerRegistrationAir> {
    pub fn owner_registration(domain: Domain) -> Result<Self, Error> {
        let air = OwnerRegistrationAir::new(domain.network).ok_or(Error::Shape)?;
        Self::new(air, Layout::OwnerRegistration, &domain.bytes())
    }
}
