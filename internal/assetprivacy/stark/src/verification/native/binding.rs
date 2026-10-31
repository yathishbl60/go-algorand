// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Error, selection::Layout};
use super::{header::Context, slot::Slot};

pub(super) fn main(context: &Context, slot: &Slot) -> Result<(), Error> {
    if slot.role != 0 || slot.output != u32::MAX {
        return Err(Error::Shape);
    }
    let start = group_start(slot.layout)?;
    for (cell, word) in slot.values[start..start + 32]
        .iter()
        .zip(context.group.as_chunks::<2>().0)
    {
        if *cell != u16::from_le_bytes(*word) {
            return Err(Error::PublicValues);
        }
    }
    Ok(())
}

fn group_start(layout: Layout) -> Result<usize, Error> {
    match layout {
        Layout::Transfer | Layout::ConfidentialTransfer => Ok(200),
        Layout::Unshield | Layout::ConfidentialUnshield => Ok(132),
        Layout::Shield | Layout::ConfidentialShield => Ok(68),
        Layout::OwnerRegistration => Ok(32),
        Layout::ViewingRegistration => Ok(1184),
        Layout::PartialPrivate => Ok(128),
        Layout::FullPrivate => Ok(96),
        Layout::PartialPublic => Ok(100),
        Layout::FullPublic => Ok(68),
        _ => Err(Error::Shape),
    }
}
