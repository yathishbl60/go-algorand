// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{
    Error,
    selection::{Layout, Prepared, Registry},
};
use super::{header::Context, slot::Slot, target::Target};
use crate::{amounts::PublicValue, spend_group_air::SpendGroupAir};

pub(super) fn build(context: &Context, registry: &Registry, slot: &Slot) -> Result<Target, Error> {
    let prepared = match slot.layout {
        Layout::ConfidentialTransfer => transfer(context, slot)?,
        Layout::ConfidentialUnshield => unshield(context, slot)?,
        _ => return Err(Error::Shape),
    };
    Ok(Target::Spend(Box::new(registry.bind(prepared)?)))
}

fn transfer(context: &Context, slot: &Slot) -> Result<Prepared<SpendGroupAir>, Error> {
    let value = PublicValue {
        debit: slot.amount(192)?,
        credit: slot.amount(196)?,
    };
    Prepared::confidential_transfer(
        context.domain,
        value,
        super::confidential::participants(slot, 264)?,
    )
}

fn unshield(context: &Context, slot: &Slot) -> Result<Prepared<SpendGroupAir>, Error> {
    Prepared::confidential_unshield(
        context.domain,
        slot.amount(128)?,
        super::confidential::participants(slot, 196)?,
    )
}
