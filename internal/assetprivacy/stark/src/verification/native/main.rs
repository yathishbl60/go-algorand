// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{
    Error,
    selection::{Layout, Prepared, Registry},
};
use super::{header::Context, slot::Slot, target::Target};
use crate::amounts::PublicValue;

pub(super) fn spend(context: &Context, registry: &Registry, slot: &Slot) -> Result<Target, Error> {
    let prepared = match slot.layout {
        Layout::Transfer => Prepared::transfer(
            context.domain,
            PublicValue {
                debit: slot.amount(192)?,
                credit: slot.amount(196)?,
            },
        )?,
        Layout::Unshield => Prepared::unshield(context.domain, slot.amount(128)?)?,
        _ => return Err(Error::Shape),
    };
    Ok(Target::Spend(Box::new(registry.bind(prepared)?)))
}
