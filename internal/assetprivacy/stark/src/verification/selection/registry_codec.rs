// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Cap, Error, implementation};
use super::{
    Entry, Layout,
    layout::ORDER,
    registry::{MAGIC, RECORD, SIZE},
};
use p3_uni_stark::PreprocessedVerifierKey;

/// Offline artifact production only. Its digest must be approved independently.
pub fn encode_registry(profile: [u8; 64], entries: &[Entry]) -> Result<Vec<u8>, Error> {
    if profile == [0; 64] || entries.len() != ORDER.len() {
        return Err(Error::Artifact);
    }
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(SIZE)
        .map_err(|_| Error::AllocationFailure)?;
    bytes.extend_from_slice(&MAGIC);
    bytes.extend_from_slice(&profile);
    bytes.extend_from_slice(&implementation::ID);
    bytes.push(12);
    for (entry, layout) in entries.iter().zip(ORDER) {
        if entry.layout != layout {
            return Err(Error::Artifact);
        }
        bytes.extend_from_slice(&encode_record(entry));
    }
    Ok(bytes)
}

fn encode_record(entry: &Entry) -> [u8; RECORD] {
    let mut bytes = layout_record(entry.layout);
    for (index, word) in entry.key.commitment.roots()[0].iter().enumerate() {
        bytes[18 + index * 8..26 + index * 8].copy_from_slice(&word.to_le_bytes());
    }
    bytes
}

fn layout_record(layout: Layout) -> [u8; RECORD] {
    let shape = layout.shape();
    let mut bytes = [0; RECORD];
    bytes[0] = layout as u8;
    for (index, value) in [layout.rows(), shape.width, shape.preprocessed, shape.public]
        .iter()
        .enumerate()
    {
        bytes[1 + index * 4..5 + index * 4].copy_from_slice(&(*value as u32).to_le_bytes());
    }
    bytes[17] = 3;
    bytes
}

pub(super) fn check_record(bytes: &[u8; RECORD], layout: Layout) -> Result<(), Error> {
    if bytes[..18] != layout_record(layout)[..18] {
        return Err(Error::Artifact);
    }
    Ok(())
}

pub(super) fn decode_record(bytes: &[u8; RECORD], layout: Layout) -> Result<Entry, Error> {
    let mut root = [0; 8];
    for (index, word) in root.iter_mut().enumerate() {
        *word = u64::from_le_bytes(
            bytes[18 + index * 8..26 + index * 8]
                .try_into()
                .map_err(|_| Error::Artifact)?,
        );
    }
    Entry::new(
        layout,
        PreprocessedVerifierKey {
            width: layout.shape().preprocessed,
            degree_bits: layout.rows().ilog2() as usize + 1,
            commitment: root_cap(root)?,
        },
    )
}

fn root_cap(root: [u64; 8]) -> Result<Cap, Error> {
    let mut roots = Vec::new();
    roots
        .try_reserve_exact(1)
        .map_err(|_| Error::AllocationFailure)?;
    roots.push(root);
    Ok(Cap::new(roots))
}
