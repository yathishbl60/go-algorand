// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{
    Error,
    selection::{Layout, Prepared, Registry},
};
use super::{header::Context, slot::Slot, target::Target};
use crate::envelope_air::EnvelopeContext;

pub(super) fn build(context: &Context, registry: &Registry, slot: &Slot) -> Result<Target, Error> {
    let envelope = context_for(context, slot)?;
    match slot.layout {
        Layout::Recipient => Ok(Target::Recipient(Box::new(
            registry.bind(Prepared::recipient(envelope)?)?,
        ))),
        Layout::Disclosure => disclosure(registry, slot, envelope),
        Layout::Retained => Ok(Target::Retained(Box::new(
            registry.bind(Prepared::retained(envelope)?)?,
        ))),
        Layout::Recovered => Ok(Target::Recovered(Box::new(
            registry.bind(Prepared::recovered(envelope)?)?,
        ))),
        _ => Err(Error::Shape),
    }
}

fn disclosure(
    registry: &Registry,
    slot: &Slot,
    envelope: EnvelopeContext,
) -> Result<Target, Error> {
    let authority = slot.bytes::<32>(1100)?;
    Ok(Target::Disclosure(Box::new(
        registry.bind(Prepared::disclosure(envelope, authority)?)?,
    )))
}

fn context_for(context: &Context, slot: &Slot) -> Result<EnvelopeContext, Error> {
    let bytes = slot.bytes::<248>(0)?;
    let domain = context.domain;
    if bytes[43..75] != domain.network
        || bytes[75..83] != domain.asset.to_be_bytes()
        || bytes[83..147] != domain.policy
        || bytes[147] != slot.role
        || bytes[180..184] != slot.output.to_be_bytes()
    {
        return Err(Error::PublicValues);
    }
    EnvelopeContext::parse(bytes).ok_or(Error::PublicValues)
}
