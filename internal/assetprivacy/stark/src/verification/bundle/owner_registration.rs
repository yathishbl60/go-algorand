// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Error, Verifier};
use super::{Expected, target::Target};
use crate::owner_registration_air::OwnerRegistrationAir;

impl<'a> Expected<'a> {
    pub fn owner_registration(
        verifier: &'a Verifier<OwnerRegistrationAir>,
        values: &'a [u16],
    ) -> Result<Self, Error> {
        Self::main(21, Target::OwnerRegistration(verifier), values)
    }
}
