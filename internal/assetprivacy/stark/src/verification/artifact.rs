// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Error;
use crate::Config;
use p3_uni_stark::PreprocessedVerifierKey;

/// Trusted, locally selected configuration. Never construct this from a proof.
/// `relation` must identify every AIR constructor argument and V2 relation revision.
pub(super) struct Artifact {
    pub(super) profile: [u8; 64],
    pub(super) registry: [u8; 64],
    pub(super) relation: [u8; 64],
    pub(super) rows: usize,
    pub(super) key: PreprocessedVerifierKey<Config>,
}

impl Artifact {
    /// Integrity binding is not security approval or a key-generation ceremony.
    pub(super) fn from_trusted(
        profile: [u8; 64],
        registry: [u8; 64],
        relation: [u8; 64],
        rows: usize,
        key: PreprocessedVerifierKey<Config>,
    ) -> Result<Self, Error> {
        if registry == [0; 64] || !Self::valid(profile, relation, rows, &key) {
            return Err(Error::Artifact);
        }
        Ok(Self {
            profile,
            registry,
            relation,
            rows,
            key,
        })
    }

    fn valid(
        profile: [u8; 64],
        relation: [u8; 64],
        rows: usize,
        key: &PreprocessedVerifierKey<Config>,
    ) -> bool {
        profile != [0; 64]
            && relation != [0; 64]
            && rows.is_power_of_two()
            && (crate::MIN_TRACE_ROWS..=65536).contains(&rows)
            && key.degree_bits == rows.ilog2() as usize + 1
            && key.commitment.num_roots() == 1
    }
}
