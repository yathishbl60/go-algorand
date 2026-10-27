// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::shape::Shape;
use super::Layout;
use crate::{
    authorized_disclosure_air as ad, authorized_recipient_air as ar, recovery_group_air as rg,
    recovery_recipient_air as rc, retained_recipient_air as rr, spend_group_air as sg,
};

impl Layout {
    pub fn rows(self) -> usize {
        match self {
            Self::Transfer | Self::Unshield => sg::ROWS,
            Self::PartialPrivate | Self::FullPrivate | Self::PartialPublic | Self::FullPublic => {
                rg::ROWS
            }
            Self::Recipient => ar::ROWS,
            Self::Disclosure => ad::ROWS,
            Self::Retained => rr::ROWS,
            Self::Recovered => rc::ROWS,
        }
    }

    pub(super) fn shape(self) -> Shape {
        let (width, preprocessed) = match self {
            Self::Transfer | Self::Unshield => (sg::WIDTH, sg::PREPROCESSED),
            Self::PartialPrivate | Self::FullPrivate | Self::PartialPublic | Self::FullPublic => {
                (rg::WIDTH, rg::PREPROCESSED)
            }
            Self::Recipient => (ar::WIDTH, ar::PREPROCESSED),
            Self::Disclosure => (ad::WIDTH, ad::PREPROCESSED),
            Self::Retained => (rr::WIDTH, rr::PREPROCESSED),
            Self::Recovered => (rc::WIDTH, rc::PREPROCESSED),
        };
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
            Self::PartialPrivate => 192,
            Self::FullPrivate => 160,
            Self::PartialPublic => 164,
            Self::FullPublic => 132,
            Self::Recipient => ar::PUBLIC_VALUES,
            Self::Disclosure => ad::PUBLIC_VALUES,
            Self::Retained => rr::PUBLIC_VALUES,
            Self::Recovered => rc::PUBLIC_VALUES,
        }
    }
}
