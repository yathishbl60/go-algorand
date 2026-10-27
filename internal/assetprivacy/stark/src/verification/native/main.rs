// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{
    Error,
    selection::{Layout, Prepared, Registry},
};
use super::{header::Context, slot::Slot, target::Target};
use crate::{amounts::PublicValue, recovery_group_air::Operation};

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

pub(super) fn recovery(
    context: &Context,
    registry: &Registry,
    slot: &Slot,
) -> Result<Target, Error> {
    let operation = match slot.layout {
        Layout::PartialPrivate => Operation::PartialPrivate,
        Layout::FullPrivate => Operation::FullPrivate,
        Layout::PartialPublic => Operation::PartialPublic {
            credit: slot.amount(96)?,
        },
        Layout::FullPublic => Operation::FullPublic {
            credit: slot.amount(64)?,
        },
        _ => return Err(Error::Shape),
    };
    let authority = slot.bytes::<32>(slot.values.len() - 32)?;
    let prepared = Prepared::recovery(context.domain, operation, authority)?;
    Ok(Target::Recovery(Box::new(registry.bind(prepared)?)))
}
