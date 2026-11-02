// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Cap, Error};
use super::{Entry, Layout, registry::RECORD};
use p3_uni_stark::PreprocessedVerifierKey;

pub use super::registry_encode::encode_registry;
pub(super) use super::registry_record::check_record;

pub(super) fn decode_record(bytes: &[u8; RECORD], layout: Layout) -> Result<Option<Entry>, Error> {
    check_record(bytes, layout)?;
    if bytes[17] == 0 {
        return Ok(None);
    }
    Ok(Some(Entry::new(layout, decode_key(bytes, layout)?)?))
}

fn decode_key(
    bytes: &[u8; RECORD],
    layout: Layout,
) -> Result<PreprocessedVerifierKey<crate::Config>, Error> {
    let mut root = [0; 8];
    for (index, word) in root.iter_mut().enumerate() {
        *word = u64::from_le_bytes(
            bytes[18 + index * 8..26 + index * 8]
                .try_into()
                .map_err(|_| Error::Artifact)?,
        );
    }
    Ok(PreprocessedVerifierKey {
        width: layout.shape().preprocessed,
        degree_bits: layout.rows().ilog2() as usize + 1,
        commitment: root_cap(root)?,
    })
}

fn root_cap(root: [u64; 8]) -> Result<Cap, Error> {
    let mut roots = Vec::new();
    roots
        .try_reserve_exact(1)
        .map_err(|_| Error::AllocationFailure)?;
    roots.push(root);
    Ok(Cap::new(roots))
}
