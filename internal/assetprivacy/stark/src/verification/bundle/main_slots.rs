// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Error, Verifier};
use super::{Expected, RecoveryKind, target::Target};
use crate::{recovery_group_air::RecoveryGroupAir, spend_group_air::SpendGroupAir};

impl<'a> Expected<'a> {
    pub fn transfer(
        verifier: &'a Verifier<SpendGroupAir>,
        values: &'a [u16],
    ) -> Result<Self, Error> {
        Self::main(1, Target::Spend(verifier), values)
    }

    pub fn unshield(
        verifier: &'a Verifier<SpendGroupAir>,
        values: &'a [u16],
    ) -> Result<Self, Error> {
        Self::main(2, Target::Spend(verifier), values)
    }

    pub fn recovery(
        kind: RecoveryKind,
        verifier: &'a Verifier<RecoveryGroupAir>,
        values: &'a [u16],
    ) -> Result<Self, Error> {
        Self::main(kind as u8, Target::Recovery(verifier), values)
    }
}
