// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::{PublicSetup, Registry};

impl Registry {
    /// Checks public setup metadata/root before private proving; not the full prover data.
    pub fn check_setup(&self, setup: &PublicSetup) -> Result<(), Error> {
        let expected = self
            .entries
            .iter()
            .find(|entry| entry.layout == setup.entry.layout)
            .ok_or(Error::Artifact)?;
        if setup.data.width != expected.key.width
            || setup.data.degree_bits != expected.key.degree_bits
            || setup.data.commitment.roots() != expected.key.commitment.roots()
        {
            return Err(Error::Artifact);
        }
        Ok(())
    }
}
