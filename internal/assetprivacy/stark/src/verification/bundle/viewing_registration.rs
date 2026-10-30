// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Error, Verifier};
use super::{Expected, target::Target};
use crate::mlkem_keygen_air::MlKemKeygenAir;

impl<'a> Expected<'a> {
    pub fn viewing_registration(
        verifier: &'a Verifier<MlKemKeygenAir>,
        values: &'a [u16],
    ) -> Result<Self, Error> {
        Self::main(22, Target::ViewingRegistration(verifier), values)
    }
}
