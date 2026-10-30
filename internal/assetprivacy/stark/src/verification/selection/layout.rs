// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Native relation kind. Equal preprocessing roots never merge distinct kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Layout {
    Transfer = 1,
    Unshield = 2,
    Shield = 3,
    PartialPrivate = 4,
    FullPrivate = 5,
    PartialPublic = 6,
    FullPublic = 7,
    Recipient = 17,
    Disclosure = 18,
    Retained = 19,
    Recovered = 20,
    OwnerRegistration = 21,
    ViewingRegistration = 22,
}

pub(super) const ORDER: [Layout; 13] = [
    Layout::Transfer,
    Layout::Unshield,
    Layout::Shield,
    Layout::PartialPrivate,
    Layout::FullPrivate,
    Layout::PartialPublic,
    Layout::FullPublic,
    Layout::Recipient,
    Layout::Disclosure,
    Layout::Retained,
    Layout::Recovered,
    Layout::OwnerRegistration,
    Layout::ViewingRegistration,
];

impl Layout {
    pub fn from_byte(kind: u8) -> Option<Self> {
        ORDER.iter().copied().find(|layout| *layout as u8 == kind)
    }
}
