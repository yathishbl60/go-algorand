// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Layout;

pub(super) enum ShapeKind {
    Spend,
    Shield,
    Recovery,
    Owner,
    Viewing,
    Recipient,
    Disclosure,
    Retained,
    Recovered,
}

impl Layout {
    pub(super) fn shape_kind(self) -> ShapeKind {
        match self {
            Self::Transfer => ShapeKind::Spend,
            Self::Unshield => ShapeKind::Spend,
            Self::Shield => ShapeKind::Shield,
            Self::PartialPrivate => ShapeKind::Recovery,
            Self::FullPrivate => ShapeKind::Recovery,
            Self::PartialPublic => ShapeKind::Recovery,
            Self::FullPublic => ShapeKind::Recovery,
            Self::ConfidentialTransfer => ShapeKind::Spend,
            Self::ConfidentialUnshield => ShapeKind::Spend,
            Self::ConfidentialShield => ShapeKind::Shield,
            Self::Recipient => ShapeKind::Recipient,
            Self::Disclosure => ShapeKind::Disclosure,
            Self::Retained => ShapeKind::Retained,
            Self::Recovered => ShapeKind::Recovered,
            Self::OwnerRegistration => ShapeKind::Owner,
            Self::ViewingRegistration => ShapeKind::Viewing,
        }
    }
}
