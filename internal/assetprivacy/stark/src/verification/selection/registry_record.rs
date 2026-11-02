// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::{Entry, Layout, registry::RECORD};

pub(super) fn layout_record(layout: Layout, flags: u8) -> [u8; RECORD] {
    let shape = layout.shape();
    let mut bytes = [0; RECORD];
    bytes[0] = layout as u8;
    for (index, value) in [layout.rows(), shape.width, shape.preprocessed, shape.public]
        .iter()
        .enumerate()
    {
        bytes[1 + index * 4..5 + index * 4].copy_from_slice(&(*value as u32).to_le_bytes());
    }
    bytes[17] = flags;
    bytes
}

pub(super) fn check_record(bytes: &[u8; RECORD], layout: Layout) -> Result<(), Error> {
    let absent = bytes[17] == 0;
    let zero_root = bytes[18..].iter().all(|byte| *byte == 0);
    if bytes[..17] != layout_record(layout, 0)[..17]
        || !matches!(bytes[17], 0 | 3)
        || absent != zero_root
    {
        return Err(Error::Artifact);
    }
    Ok(())
}

pub(super) fn encode_record(entry: &Entry) -> Result<[u8; RECORD], Error> {
    let mut bytes = layout_record(entry.layout, 3);
    for (index, word) in entry.key.commitment.roots()[0].iter().enumerate() {
        bytes[18 + index * 8..26 + index * 8].copy_from_slice(&word.to_le_bytes());
    }
    check_record(&bytes, entry.layout)?;
    Ok(bytes)
}
