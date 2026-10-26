// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::{Layout, Prepared};
use crate::{
    authorized_disclosure_air::AuthorizedDisclosureAir,
    authorized_recipient_air::AuthorizedRecipientAir, envelope_air::EnvelopeContext,
    recovery_recipient_air::RecoveryRecipientAir, retained_recipient_air::RetainedRecipientAir,
};

impl Prepared<AuthorizedRecipientAir> {
    pub fn recipient(context: EnvelopeContext) -> Result<Self, Error> {
        let air = AuthorizedRecipientAir::new(context).ok_or(Error::Shape)?;
        Self::new(air, Layout::Recipient, context.bytes())
    }
}

impl Prepared<AuthorizedDisclosureAir> {
    pub fn disclosure(context: EnvelopeContext, authority: [u8; 32]) -> Result<Self, Error> {
        let air = AuthorizedDisclosureAir::new(context, authority).ok_or(Error::Shape)?;
        let mut args = context.bytes().to_vec();
        args.extend_from_slice(&authority);
        Self::new(air, Layout::Disclosure, &args)
    }
}

impl Prepared<RetainedRecipientAir> {
    pub fn retained(context: EnvelopeContext) -> Result<Self, Error> {
        let air = RetainedRecipientAir::new(context).ok_or(Error::Shape)?;
        Self::new(air, Layout::Retained, context.bytes())
    }
}

impl Prepared<RecoveryRecipientAir> {
    pub fn recovered(context: EnvelopeContext) -> Result<Self, Error> {
        let air = RecoveryRecipientAir::new(context).ok_or(Error::Shape)?;
        Self::new(air, Layout::Recovered, context.bytes())
    }
}
