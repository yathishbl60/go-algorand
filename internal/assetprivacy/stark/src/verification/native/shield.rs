// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{
    Error,
    selection::{Prepared, Registry},
};
use super::{header::Context, slot::Slot, target::Target};

pub(super) fn build(context: &Context, registry: &Registry, slot: &Slot) -> Result<Target, Error> {
    let prepared = Prepared::shield(context.domain, slot.amount(64)?)?;
    Ok(Target::Shield(Box::new(registry.bind(prepared)?)))
}
