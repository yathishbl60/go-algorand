// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::{Domain, Layout, Prepared};
use crate::mlkem_keygen_air::MlKemKeygenAir;

impl Prepared<MlKemKeygenAir> {
    pub fn viewing_registration(domain: Domain, public_key: [u8; 1184]) -> Result<Self, Error> {
        let air = MlKemKeygenAir::new(public_key).ok_or(Error::PublicValues)?;
        let mut args = [0; 1288];
        args[..104].copy_from_slice(&domain.bytes());
        args[104..].copy_from_slice(&public_key);
        Self::new(air, Layout::ViewingRegistration, &args)
    }
}
