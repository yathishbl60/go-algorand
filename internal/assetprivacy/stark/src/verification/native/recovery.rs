// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{
    Error,
    selection::{Layout, Prepared, Registry},
};
use super::{header::Context, slot::Slot, target::Target};
use crate::recovery_group_air::Operation;

pub(super) fn build(context: &Context, registry: &Registry, slot: &Slot) -> Result<Target, Error> {
    let (operation, prefix) = operation(slot)?;
    let authority = slot.bytes::<32>(prefix - 32)?;
    let prepared = if slot.values.len() == prefix {
        Prepared::recovery(context.domain, operation, authority)?
    } else {
        Prepared::confidential_recovery(
            context.domain,
            operation,
            authority,
            super::confidential::participants(slot, prefix)?,
        )?
    };
    Ok(Target::Recovery(Box::new(registry.bind(prepared)?)))
}

fn operation(slot: &Slot) -> Result<(Operation, usize), Error> {
    match slot.layout {
        Layout::PartialPrivate | Layout::ConfidentialPartialPrivate => {
            Ok((Operation::PartialPrivate, 192))
        }
        Layout::FullPrivate | Layout::ConfidentialFullPrivate => Ok((Operation::FullPrivate, 160)),
        _ => public_operation(slot),
    }
}

fn public_operation(slot: &Slot) -> Result<(Operation, usize), Error> {
    let credit = credit(slot)?;
    match slot.layout {
        Layout::PartialPublic | Layout::ConfidentialPartialPublic => {
            Ok((Operation::PartialPublic { credit }, 164))
        }
        Layout::FullPublic | Layout::ConfidentialFullPublic => {
            Ok((Operation::FullPublic { credit }, 132))
        }
        _ => Err(Error::Shape),
    }
}

fn credit(slot: &Slot) -> Result<u64, Error> {
    match slot.layout {
        Layout::PartialPublic | Layout::ConfidentialPartialPublic => slot.amount(96),
        Layout::FullPublic | Layout::ConfidentialFullPublic => slot.amount(64),
        _ => Err(Error::Shape),
    }
}
