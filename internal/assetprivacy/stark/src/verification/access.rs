// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DecodeLimits, Error, Verifier, decode};

impl<A> Verifier<A> {
    pub fn artifact_id(&self) -> [u8; 64] {
        self.identity
    }
    pub(super) fn public_values_len(&self) -> usize {
        self.shape.public
    }
    pub(super) fn profile_id(&self) -> [u8; 64] {
        self.artifact.profile
    }
    pub(super) fn registry_id(&self) -> [u8; 64] {
        self.artifact.registry
    }

    pub(super) fn check_header(&self, bytes: &[u8], limits: DecodeLimits) -> Result<(), Error> {
        if bytes.len() > limits.wire {
            return Err(Error::WireLimit);
        }
        if bytes.len() < 73 {
            return Err(Error::Truncated);
        }
        if bytes[..8] != decode::MAGIC || bytes[8..72] != self.identity {
            return Err(Error::Artifact);
        }
        if usize::from(bytes[72]) != self.artifact.key.degree_bits {
            return Err(Error::Shape);
        }
        Ok(())
    }
}
