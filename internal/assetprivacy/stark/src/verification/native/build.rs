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
            Layout::OwnerRegistration => super::owner_registration::build(context, registry),
            Layout::ViewingRegistration => {
                super::viewing_registration::build(context, registry, slot)
            }
            Layout::Recipient | Layout::Disclosure | Layout::Retained | Layout::Recovered => {
                super::output::build(context, registry, slot)
            }
            _ => super::main_build::build(context, registry, slot),
        }
    }
}
