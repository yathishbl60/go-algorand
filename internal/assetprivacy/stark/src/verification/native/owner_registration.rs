// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{
    Error,
    selection::{Prepared, Registry},
};
use super::{header::Context, target::Target};

pub(super) fn build(context: &Context, registry: &Registry) -> Result<Target, Error> {
    let prepared = Prepared::owner_registration(context.domain)?;
    Ok(Target::OwnerRegistration(Box::new(
        registry.bind(prepared)?,
    )))
}
