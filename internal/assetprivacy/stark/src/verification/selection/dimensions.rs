// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::shape::Shape;
use super::{Layout, dimensions_other::ShapeKind};
use crate::{
    authorized_disclosure_air as ad, authorized_recipient_air as ar, mlkem_keygen_air as view,
    owner_registration_air as own, recovery_group_air as rg, recovery_recipient_air as rc,
    retained_recipient_air as rr, shield_group_air as sh, spend_group_air as sg,
};

impl Layout {
    pub fn rows(self) -> usize {
        match self.shape_kind() {
            ShapeKind::Spend => sg::ROWS,
            ShapeKind::Shield => sh::ROWS,
            ShapeKind::Recovery => rg::ROWS,
            ShapeKind::Owner => own::ROWS,
            ShapeKind::Viewing => view::ROWS,
            ShapeKind::Recipient => ar::ROWS,
            ShapeKind::Disclosure => ad::ROWS,
            ShapeKind::Retained => rr::ROWS,
            ShapeKind::Recovered => rc::ROWS,
        }
    }
    fn dimensions(self) -> (usize, usize) {
        match self.shape_kind() {
            ShapeKind::Spend => (sg::WIDTH, sg::PREPROCESSED),
            ShapeKind::Shield => (sh::WIDTH, sh::PREPROCESSED),
            ShapeKind::Recovery => (rg::WIDTH, rg::PREPROCESSED),
            ShapeKind::Owner => (own::WIDTH, own::PREPROCESSED),
            ShapeKind::Viewing => (view::WIDTH, view::PREPROCESSED),
            ShapeKind::Recipient => (ar::WIDTH, ar::PREPROCESSED),
            ShapeKind::Disclosure => (ad::WIDTH, ad::PREPROCESSED),
            ShapeKind::Retained => (rr::WIDTH, rr::PREPROCESSED),
            ShapeKind::Recovered => (rc::WIDTH, rc::PREPROCESSED),
        }
    }
    pub(super) fn shape(self) -> Shape {
        let (width, preprocessed) = self.dimensions();
        Shape {
            width,
            preprocessed,
            public: self.public_count(),
            main_next: true,
            preprocessed_next: true,
        }
    }
    pub fn public_count(self) -> usize {
        match self {
            Self::Transfer => sg::TRANSFER_PUBLIC_VALUES,
            Self::Unshield => sg::UNSHIELD_PUBLIC_VALUES,
            Self::Shield => sh::PUBLIC_VALUES,
            Self::PartialPrivate => 192,
            Self::FullPrivate => 160,
            Self::PartialPublic => 164,
            Self::FullPublic => 132,
            Self::ConfidentialTransfer => sg::CONFIDENTIAL_TRANSFER_PUBLIC_VALUES,
            Self::ConfidentialUnshield => sg::CONFIDENTIAL_UNSHIELD_PUBLIC_VALUES,
            Self::ConfidentialShield => sh::CONFIDENTIAL_PUBLIC_VALUES,
            Self::Recipient => ar::PUBLIC_VALUES,
            Self::Disclosure => ad::PUBLIC_VALUES,
            Self::Retained => rr::PUBLIC_VALUES,
            Self::Recovered => rc::PUBLIC_VALUES,
            Self::OwnerRegistration => own::PUBLIC_VALUES,
            Self::ViewingRegistration => view::PUBLIC_VALUES,
        }
    }
}
