// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{DecodeLimits, Error, Verifier};
use crate::mlkem_keygen_air::MlKemKeygenAir;
use crate::owner_registration_air::OwnerRegistrationAir;
use crate::{
    authorized_disclosure_air::AuthorizedDisclosureAir,
    authorized_recipient_air::AuthorizedRecipientAir, recovery_group_air::RecoveryGroupAir,
    recovery_recipient_air::RecoveryRecipientAir, retained_recipient_air::RetainedRecipientAir,
    shield_group_air::ShieldGroupAir, spend_group_air::SpendGroupAir,
};

#[derive(Clone, Copy)]
pub(super) enum Target<'a> {
    Shield(&'a Verifier<ShieldGroupAir>),
    OwnerRegistration(&'a Verifier<OwnerRegistrationAir>),
    ViewingRegistration(&'a Verifier<MlKemKeygenAir>),
    Spend(&'a Verifier<SpendGroupAir>),
    Recovery(&'a Verifier<RecoveryGroupAir>),
    Recipient(&'a Verifier<AuthorizedRecipientAir>),
    Disclosure(&'a Verifier<AuthorizedDisclosureAir>),
    Retained(&'a Verifier<RetainedRecipientAir>),
    Recovered(&'a Verifier<RecoveryRecipientAir>),
}

impl Target<'_> {
    pub(super) fn header(&self, proof: &[u8], limits: DecodeLimits) -> Result<(), Error> {
        match self {
            Self::Shield(v) => v.check_header(proof, limits),
            Self::OwnerRegistration(v) => v.check_header(proof, limits),
            Self::ViewingRegistration(v) => v.check_header(proof, limits),
            Self::Spend(v) => v.check_header(proof, limits),
            Self::Recovery(v) => v.check_header(proof, limits),
            Self::Recipient(v) => v.check_header(proof, limits),
            Self::Disclosure(v) => v.check_header(proof, limits),
            Self::Retained(v) => v.check_header(proof, limits),
            Self::Recovered(v) => v.check_header(proof, limits),
        }
    }

    pub(super) fn verify(
        &self,
        proof: &[u8],
        values: &[u16],
        limits: DecodeLimits,
    ) -> Result<(), Error> {
        match self {
            Self::Shield(v) => v.verify(proof, values, limits),
            Self::OwnerRegistration(v) => v.verify(proof, values, limits),
            Self::ViewingRegistration(v) => v.verify(proof, values, limits),
            Self::Spend(v) => v.verify(proof, values, limits),
            Self::Recovery(v) => v.verify(proof, values, limits),
            Self::Recipient(v) => v.verify(proof, values, limits),
            Self::Disclosure(v) => v.verify(proof, values, limits),
            Self::Retained(v) => v.verify(proof, values, limits),
            Self::Recovered(v) => v.verify(proof, values, limits),
        }
    }

    pub(super) fn public_len(&self) -> usize {
        match self {
            Self::Shield(v) => v.public_values_len(),
            Self::OwnerRegistration(v) => v.public_values_len(),
            Self::ViewingRegistration(v) => v.public_values_len(),
            Self::Spend(v) => v.public_values_len(),
            Self::Recovery(v) => v.public_values_len(),
            Self::Recipient(v) => v.public_values_len(),
            Self::Disclosure(v) => v.public_values_len(),
            Self::Retained(v) => v.public_values_len(),
            Self::Recovered(v) => v.public_values_len(),
        }
    }
}
