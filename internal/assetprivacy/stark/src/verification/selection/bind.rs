// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Artifact, Error, Verifier, artifact::Identity};
use super::{Prepared, Registry};
use crate::Value;
use p3_air::BaseAir;

impl Registry {
    /// Only prepared native relations and this independently pinned registry select keys.
    pub fn bind<A: BaseAir<Value>>(&self, prepared: Prepared<A>) -> Result<Verifier<A>, Error> {
        let entry = self
            .entries
            .iter()
            .find(|entry| entry.layout == prepared.layout)
            .ok_or(Error::Artifact)?;
        let identity = Identity {
            profile: self.profile,
            registry: self.digest(),
            reference: self.reference_id(),
            relation: prepared.relation_id(self),
        };
        let artifact = Artifact::from_trusted(identity, prepared.layout.rows(), entry.key.clone())?;
        Verifier::from_trusted(prepared.air, artifact)
    }
}
