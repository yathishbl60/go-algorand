// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::{Entry, layout::ORDER};

pub(super) const HEADER: usize = 134;
pub(super) const RECORD: usize = 82;
pub(super) const SIZE: usize = HEADER + RECORD * 13;
pub(super) const MAGIC: [u8; 5] = *b"NAPK\x04";

/// Independent approved inputs. Never obtain these values from the artifact/witness.
#[derive(Clone, Copy)]
pub struct Pin {
    pub profile: [u8; 64],
    pub implementation: [u8; 64],
    pub digest: [u8; 64],
}

pub struct Registry {
    pub(super) profile: [u8; 64],
    digest: [u8; 64],
    reference: [u8; 64],
    pub(super) entries: Vec<Entry>,
}

impl Registry {
    pub fn load(bytes: &[u8], pin: Pin) -> Result<Self, Error> {
        preflight(bytes, pin)?;
        let mut entries = Vec::new();
        entries
            .try_reserve_exact(ORDER.len())
            .map_err(|_| Error::AllocationFailure)?;
        for (record, layout) in bytes[HEADER..].as_chunks::<RECORD>().0.iter().zip(ORDER) {
            entries.push(super::registry_codec::decode_record(record, layout)?);
        }
        Ok(Self {
            profile: pin.profile,
            digest: pin.digest,
            reference: pin.implementation,
            entries,
        })
    }

    pub fn profile_id(&self) -> [u8; 64] {
        self.profile
    }
    pub fn reference_id(&self) -> [u8; 64] {
        self.reference
    }

    pub fn digest(&self) -> [u8; 64] {
        self.digest
    }
}

fn preflight(bytes: &[u8], pin: Pin) -> Result<(), Error> {
    if !valid_pin(pin) || bytes.len() != SIZE {
        return Err(Error::Artifact);
    }
    if bytes[..5] != MAGIC
        || bytes[5..69] != pin.profile
        || bytes[69..133] != pin.implementation
        || bytes[133] != 13
        || super::registry_digest::registry_digest(bytes)? != pin.digest
    {
        return Err(Error::Artifact);
    }
    for (record, layout) in bytes[HEADER..].as_chunks::<RECORD>().0.iter().zip(ORDER) {
        super::registry_codec::check_record(record, layout)?;
    }
    Ok(())
}

fn valid_pin(pin: Pin) -> bool {
    pin.profile != [0; 64]
        && pin.implementation != [0; 64]
        && pin.digest != [0; 64]
        && super::reference::reference_supported(&pin.implementation)
}
