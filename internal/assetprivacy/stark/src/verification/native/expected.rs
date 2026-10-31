// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{
    Error,
    bundle::{DisclosureRole, Expected, RecoveryKind},
    selection::Layout,
};
use super::{slot::Slot, target::Target};

impl Target {
    pub(super) fn expected<'a>(&'a self, slot: &'a Slot) -> Result<Expected<'a>, Error> {
        let values = &slot.values;
        match self {
            Self::Shield(v) => super::expected_main::shield(slot.layout, v, values),
            Self::OwnerRegistration(v) => Expected::owner_registration(v, values),
            Self::ViewingRegistration(v) => Expected::viewing_registration(v, values),
            Self::Spend(v) => super::expected_main::spend(slot.layout, v, values),
            Self::Recovery(v) => Expected::recovery(recovery(slot.layout)?, v, values),
            Self::Recipient(v) => Expected::recipient(slot.output, v, values),
            Self::Disclosure(v) => {
                Expected::disclosure(slot.output, disclosure(slot.role)?, v, values)
            }
            Self::Retained(v) => Expected::retained(slot.output, v, values),
            Self::Recovered(v) => Expected::recovered(slot.output, v, values),
        }
    }
}

fn recovery(layout: Layout) -> Result<RecoveryKind, Error> {
    match layout {
        Layout::PartialPrivate => Ok(RecoveryKind::PartialPrivate),
        Layout::FullPrivate => Ok(RecoveryKind::FullPrivate),
        Layout::PartialPublic => Ok(RecoveryKind::PartialPublic),
        Layout::FullPublic => Ok(RecoveryKind::FullPublic),
        _ => Err(Error::Shape),
    }
}

fn disclosure(role: u8) -> Result<DisclosureRole, Error> {
    match role {
        2 => Ok(DisclosureRole::Auditor),
        3 => Ok(DisclosureRole::Controller),
        _ => Err(Error::Shape),
    }
}
