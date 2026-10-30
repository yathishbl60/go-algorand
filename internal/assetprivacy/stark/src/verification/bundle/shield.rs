// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Error, Verifier};
use super::{Expected, target::Target};
use crate::shield_group_air::ShieldGroupAir;

impl<'a> Expected<'a> {
    pub fn shield(
        verifier: &'a Verifier<ShieldGroupAir>,
        values: &'a [u16],
    ) -> Result<Self, Error> {
        Self::main(3, Target::Shield(verifier), values)
    }
}
