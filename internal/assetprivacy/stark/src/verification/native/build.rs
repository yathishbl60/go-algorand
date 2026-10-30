// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{
    Error,
    selection::{Layout, Registry},
};
use super::{header::Context, slot::Slot, target::Target};

impl Target {
    pub(super) fn build(
        context: &Context,
        registry: &Registry,
        slot: &Slot,
    ) -> Result<Self, Error> {
        match slot.layout {
            Layout::Shield => super::shield::build(context, registry, slot),
            Layout::OwnerRegistration => super::owner_registration::build(context, registry),
            Layout::Transfer | Layout::Unshield => super::main::spend(context, registry, slot),
            Layout::PartialPrivate
            | Layout::FullPrivate
            | Layout::PartialPublic
            | Layout::FullPublic => super::main::recovery(context, registry, slot),
            _ => super::output::build(context, registry, slot),
        }
    }
}
