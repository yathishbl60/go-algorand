// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::{Prepared, PublicSetup, Registry};
use crate::Value;
use p3_air::BaseAir;

impl<A: BaseAir<Value>> Prepared<A> {
    pub(super) fn check_proving_inputs(
        &self,
        registry: &Registry,
        setup: &PublicSetup,
        public_values: &[u16],
    ) -> Result<(), Error> {
        if setup.entry().layout() != self.layout {
            return Err(Error::Artifact);
        }
        if public_values.len() != self.layout.public_count() {
            return Err(Error::PublicValues);
        }
        registry.check_setup(setup)
    }
}
