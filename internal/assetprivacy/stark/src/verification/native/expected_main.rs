// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Error, Verifier, bundle::Expected, selection::Layout};
use crate::{shield_group_air::ShieldGroupAir, spend_group_air::SpendGroupAir};

pub(super) fn spend<'a>(
    layout: Layout,
    verifier: &'a Verifier<SpendGroupAir>,
    values: &'a [u16],
) -> Result<Expected<'a>, Error> {
    match layout {
        Layout::Transfer => Expected::transfer(verifier, values),
        Layout::Unshield => Expected::unshield(verifier, values),
        Layout::ConfidentialTransfer => Expected::confidential_transfer(verifier, values),
        Layout::ConfidentialUnshield => Expected::confidential_unshield(verifier, values),
        _ => Err(Error::Shape),
    }
}

pub(super) fn shield<'a>(
    layout: Layout,
    verifier: &'a Verifier<ShieldGroupAir>,
    values: &'a [u16],
) -> Result<Expected<'a>, Error> {
    match layout {
        Layout::Shield => Expected::shield(verifier, values),
        Layout::ConfidentialShield => Expected::confidential_shield(verifier, values),
        _ => Err(Error::Shape),
    }
}
