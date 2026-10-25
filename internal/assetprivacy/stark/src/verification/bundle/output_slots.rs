// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Error, Verifier};
use super::{DisclosureRole, Expected, target::Target};
use crate::{
    authorized_disclosure_air::AuthorizedDisclosureAir,
    authorized_recipient_air::AuthorizedRecipientAir, recovery_recipient_air::RecoveryRecipientAir,
    retained_recipient_air::RetainedRecipientAir,
};

impl<'a> Expected<'a> {
    pub fn recipient(
        output: u32,
        verifier: &'a Verifier<AuthorizedRecipientAir>,
        values: &'a [u16],
    ) -> Result<Self, Error> {
        Self::new(17, output, 1, Target::Recipient(verifier), values)
    }

    pub fn disclosure(
        output: u32,
        role: DisclosureRole,
        verifier: &'a Verifier<AuthorizedDisclosureAir>,
        values: &'a [u16],
    ) -> Result<Self, Error> {
        Self::new(18, output, role as u8, Target::Disclosure(verifier), values)
    }

    pub fn retained(
        output: u32,
        verifier: &'a Verifier<RetainedRecipientAir>,
        values: &'a [u16],
    ) -> Result<Self, Error> {
        Self::new(19, output, 1, Target::Retained(verifier), values)
    }

    pub fn recovered(
        output: u32,
        verifier: &'a Verifier<RecoveryRecipientAir>,
        values: &'a [u16],
    ) -> Result<Self, Error> {
        Self::new(20, output, 1, Target::Recovered(verifier), values)
    }
}
