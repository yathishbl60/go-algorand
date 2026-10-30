// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Verifier;
use crate::authorized_disclosure_air::AuthorizedDisclosureAir;
use crate::authorized_recipient_air::AuthorizedRecipientAir;
use crate::owner_registration_air::OwnerRegistrationAir;
use crate::recovery_group_air::RecoveryGroupAir;
use crate::recovery_recipient_air::RecoveryRecipientAir;
use crate::retained_recipient_air::RetainedRecipientAir;
use crate::shield_group_air::ShieldGroupAir;
use crate::spend_group_air::SpendGroupAir;

pub(super) enum Target {
    Shield(Box<Verifier<ShieldGroupAir>>),
    OwnerRegistration(Box<Verifier<OwnerRegistrationAir>>),
    Spend(Box<Verifier<SpendGroupAir>>),
    Recovery(Box<Verifier<RecoveryGroupAir>>),
    Recipient(Box<Verifier<AuthorizedRecipientAir>>),
    Disclosure(Box<Verifier<AuthorizedDisclosureAir>>),
    Retained(Box<Verifier<RetainedRecipientAir>>),
    Recovered(Box<Verifier<RecoveryRecipientAir>>),
}
