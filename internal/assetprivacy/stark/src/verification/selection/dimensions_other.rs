// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Layout::{self, *};

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
            Transfer | Unshield | ConfidentialTransfer | ConfidentialUnshield => ShapeKind::Spend,
            Shield | ConfidentialShield => ShapeKind::Shield,
            PartialPrivate
            | FullPrivate
            | PartialPublic
            | FullPublic
            | ConfidentialPartialPrivate
            | ConfidentialFullPrivate
            | ConfidentialPartialPublic
            | ConfidentialFullPublic => ShapeKind::Recovery,
            Recipient => ShapeKind::Recipient,
            Disclosure => ShapeKind::Disclosure,
            Retained => ShapeKind::Retained,
            Recovered => ShapeKind::Recovered,
            OwnerRegistration => ShapeKind::Owner,
            ViewingRegistration => ShapeKind::Viewing,
        }
    }
}
