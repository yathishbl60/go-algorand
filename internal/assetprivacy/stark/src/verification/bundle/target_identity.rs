// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::target::Target;

impl Target<'_> {
    pub(super) fn profile(&self) -> [u8; 64] {
        match self {
            Self::Shield(v) => v.profile_id(),
            Self::OwnerRegistration(v) => v.profile_id(),
            Self::ViewingRegistration(v) => v.profile_id(),
            Self::Spend(v) => v.profile_id(),
            Self::Recovery(v) => v.profile_id(),
            Self::Recipient(v) => v.profile_id(),
            Self::Disclosure(v) => v.profile_id(),
            Self::Retained(v) => v.profile_id(),
            Self::Recovered(v) => v.profile_id(),
        }
    }

    pub(super) fn registry(&self) -> [u8; 64] {
        match self {
            Self::Shield(v) => v.registry_id(),
            Self::OwnerRegistration(v) => v.registry_id(),
            Self::ViewingRegistration(v) => v.registry_id(),
            Self::Spend(v) => v.registry_id(),
            Self::Recovery(v) => v.registry_id(),
            Self::Recipient(v) => v.registry_id(),
            Self::Disclosure(v) => v.registry_id(),
            Self::Retained(v) => v.registry_id(),
            Self::Recovered(v) => v.registry_id(),
        }
    }
}
