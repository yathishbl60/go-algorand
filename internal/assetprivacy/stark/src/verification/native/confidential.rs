// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{
    Error,
    selection::{Layout, Prepared, Registry},
};
use super::{header::Context, slot::Slot, target::Target};

pub(super) fn build(context: &Context, registry: &Registry, slot: &Slot) -> Result<Target, Error> {
    if slot.layout == Layout::ConfidentialShield {
        let prepared = Prepared::confidential_shield(
            context.domain,
            slot.amount(64)?,
            participants(slot, 100)?,
        )?;
        return Ok(Target::Shield(Box::new(registry.bind(prepared)?)));
    }
    super::confidential_spend::build(context, registry, slot)
}

pub(super) fn participants(slot: &Slot, start: usize) -> Result<[[u8; 32]; 4], Error> {
    let mut accounts = [[0; 32]; 4];
    for (index, account) in accounts.iter_mut().enumerate() {
        *account = slot.bytes::<32>(start + index * 32)?;
    }
    Ok(accounts)
}
