// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    envelope_air::EnvelopeInputs,
    membership_air::MembershipPath,
    registered_key_air::{KeyMaterial, RegisteredKeyInput},
};

#[derive(Clone, Copy)]
pub struct RetainedInput<'a> {
    pub envelope: EnvelopeInputs<'a>,
    pub view_created: u64,
    pub view_path: &'a MembershipPath,
}

impl<'a> RetainedInput<'a> {
    pub(super) fn account(&self) -> [u8; 32] {
        self.envelope.opening[137..169]
            .try_into()
            .expect("note account")
    }
    pub(super) fn view(&self) -> RegisteredKeyInput<'a> {
        RegisteredKeyInput {
            material: KeyMaterial::View(self.envelope.public_key),
            account: self.account(),
            epoch: self.envelope.epoch,
            created: self.view_created,
            path: self.view_path,
        }
    }
}
