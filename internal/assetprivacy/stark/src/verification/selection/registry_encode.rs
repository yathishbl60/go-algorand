// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Error, implementation};
use super::{
    Entry,
    layout::ORDER,
    registry::{MAGIC, SIZE},
    registry_record,
};

/// Encode a nonempty canonical-order subset of actual public preprocessing keys.
/// Missing kinds are explicit unsupported capabilities, never placeholder keys.
pub fn encode_registry(profile: [u8; 64], entries: &[Entry]) -> Result<Vec<u8>, Error> {
    if profile == [0; 64] || entries.is_empty() || entries.len() > ORDER.len() {
        return Err(Error::Artifact);
    }
    let mut bytes = header(profile)?;
    let mut selected = entries.iter().peekable();
    for layout in ORDER {
        let record = if selected.peek().is_some_and(|entry| entry.layout == layout) {
            registry_record::encode_record(selected.next().ok_or(Error::Artifact)?)?
        } else {
            registry_record::layout_record(layout, 0)
        };
        bytes.extend_from_slice(&record);
    }
    if selected.next().is_some() {
        return Err(Error::Artifact);
    }
    Ok(bytes)
}

fn header(profile: [u8; 64]) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(SIZE)
        .map_err(|_| Error::AllocationFailure)?;
    bytes.extend_from_slice(&MAGIC);
    bytes.extend_from_slice(&profile);
    bytes.extend_from_slice(&implementation::ID);
    bytes.push(20);
    Ok(bytes)
}
