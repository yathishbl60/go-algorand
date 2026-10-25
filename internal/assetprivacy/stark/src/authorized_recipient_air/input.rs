// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    current_eligibility_air::EligibilityInput,
    envelope_air::EnvelopeInputs,
    membership_air::MembershipPath,
    registered_key_air::{KeyMaterial, RegisteredKeyInput},
};

#[derive(Clone, Copy)]
pub struct RecipientInput<'a> {
    pub envelope: EnvelopeInputs<'a>,
    pub view_created: u64,
    pub view_path: &'a MembershipPath,
    pub spend_epoch: u64,
    pub spend_created: u64,
    pub spend_path: &'a MembershipPath,
    pub eligibility_revision: u64,
    pub eligibility_updated: u64,
    pub eligibility_path: &'a MembershipPath,
}

impl<'a> RecipientInput<'a> {
    pub(super) fn eligibility(&self) -> EligibilityInput<'a> {
        EligibilityInput {
            account: self.account(),
            revision: self.eligibility_revision,
            updated: self.eligibility_updated,
            path: self.eligibility_path,
        }
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
    pub(super) fn spend(&self) -> RegisteredKeyInput<'a> {
        RegisteredKeyInput {
            material: KeyMaterial::Spend(self.owner()),
            account: self.account(),
            epoch: self.spend_epoch,
            created: self.spend_created,
            path: self.spend_path,
        }
    }
    pub(super) fn account(&self) -> [u8; 32] {
        self.envelope.opening[137..169]
            .try_into()
            .expect("note account")
    }
    pub(super) fn owner(&self) -> [u8; 64] {
        self.envelope.opening[73..137]
            .try_into()
            .expect("note owner")
    }
}
