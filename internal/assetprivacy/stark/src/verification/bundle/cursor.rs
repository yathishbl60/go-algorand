// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Error, decode};
use super::{BundleLimits, ENTRY, Expected, HEADER, MAGIC};

pub(super) struct Cursor<'a> {
    remaining: &'a [u8],
}

impl<'a> Cursor<'a> {
    pub(super) fn start(
        bytes: &'a [u8],
        slots: &[Expected<'_>],
        limits: BundleLimits,
    ) -> Result<Self, Error> {
        super::layout::check(slots)?;
        if bytes.len() < HEADER || bytes.len() > limits.wire {
            return Err(Error::WireLimit);
        }
        if bytes[..5] != MAGIC
            || usize::from(u16::from_be_bytes([bytes[5], bytes[6]])) != slots.len()
        {
            return Err(Error::Shape);
        }
        Ok(Self {
            remaining: &bytes[HEADER..],
        })
    }

    pub(super) fn next(
        &mut self,
        slot: &Expected<'_>,
        limits: BundleLimits,
    ) -> Result<&'a [u8], Error> {
        let (header, rest) = self
            .remaining
            .split_at_checked(ENTRY)
            .ok_or(Error::Truncated)?;
        let length = read_length(header, slot)?;
        if length > limits.proof {
            return Err(Error::WireLimit);
        }
        let (proof, remaining) = rest.split_at_checked(length).ok_or(Error::Truncated)?;
        limits.proof(proof)?;
        self.remaining = remaining;
        Ok(proof)
    }

    pub(super) fn finish(self) -> Result<(), Error> {
        if !self.remaining.is_empty() {
            return Err(Error::NonCanonical);
        }
        Ok(())
    }
}

fn read_length(header: &[u8], slot: &Expected<'_>) -> Result<usize, Error> {
    let output = u32::from_be_bytes(header[1..5].try_into().map_err(|_| Error::Truncated)?);
    if (header[0], output, header[5]) != slot.descriptor() {
        return Err(Error::Shape);
    }
    Ok(u32::from_be_bytes(header[6..10].try_into().map_err(|_| Error::Truncated)?) as usize)
}

pub(super) fn stark_header(bytes: &[u8]) -> Result<(), Error> {
    if bytes.len() < 73 || bytes[..8] != decode::MAGIC {
        return Err(Error::NonCanonical);
    }
    Ok(())
}
