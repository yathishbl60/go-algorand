// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Error, Verifier};
use super::{Expected, target::Target};
use crate::{shield_group_air::ShieldGroupAir, spend_group_air::SpendGroupAir};

impl<'a> Expected<'a> {
    pub fn confidential_transfer(
        verifier: &'a Verifier<SpendGroupAir>,
        values: &'a [u16],
    ) -> Result<Self, Error> {
        Self::main(8, Target::Spend(verifier), values)
    }
    pub fn confidential_unshield(
        verifier: &'a Verifier<SpendGroupAir>,
        values: &'a [u16],
    ) -> Result<Self, Error> {
        Self::main(9, Target::Spend(verifier), values)
    }
    pub fn confidential_shield(
        verifier: &'a Verifier<ShieldGroupAir>,
        values: &'a [u16],
    ) -> Result<Self, Error> {
        Self::main(10, Target::Shield(verifier), values)
    }
}
