// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::registry::SIZE;
use sha3::{Digest, Sha3_512};

/// Bounded file integrity digest; computing it does not approve a registry.
pub fn registry_digest(bytes: &[u8]) -> Result<[u8; 64], Error> {
    if bytes.len() != SIZE {
        return Err(Error::Artifact);
    }
    let mut hash = Sha3_512::new();
    hash.update(b"Algorand/AssetPrivacy/Candidate1/ArtifactRegistryV2/");
    hash.update(bytes);
    Ok(hash.finalize().into())
}
