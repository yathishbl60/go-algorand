// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    envelope_air::EnvelopeInputs,
    membership_air::MembershipPath,
    registered_key_air::{KeyMaterial, RegisteredKeyInput},
};

#[derive(Clone, Copy)]
pub struct DisclosureInput<'a> {
    pub envelope: EnvelopeInputs<'a>,
    pub view_created: u64,
    pub view_path: &'a MembershipPath,
}

impl<'a> DisclosureInput<'a> {
    pub(super) fn view(&self, authority: [u8; 32]) -> RegisteredKeyInput<'a> {
        RegisteredKeyInput {
            material: KeyMaterial::View(self.envelope.public_key),
            account: authority,
            epoch: self.envelope.epoch,
            created: self.view_created,
            path: self.view_path,
        }
    }
}
