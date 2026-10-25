// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DecodeLimits, Error};

pub(super) struct Writer {
    bytes: Vec<u8>,
    limit: usize,
}

impl Writer {
    pub(super) fn new(limits: DecodeLimits) -> Self {
        Self {
            bytes: Vec::new(),
            limit: limits.wire.min(limits.allocation),
        }
    }

    fn reserve(&mut self, end: usize) -> Result<(), Error> {
        if end > self.limit {
            return Err(Error::WireLimit);
        }
        if end > self.bytes.capacity() {
            let target = end
                .max(self.bytes.capacity().saturating_mul(2))
                .min(self.limit);
            self.bytes
                .try_reserve_exact(target - self.bytes.len())
                .map_err(|_| Error::MemoryLimit)?;
        }
        Ok(())
    }

    pub(super) fn push(&mut self, bytes: &[u8]) -> Result<(), Error> {
        let end = self
            .bytes
            .len()
            .checked_add(bytes.len())
            .ok_or(Error::WireLimit)?;
        self.reserve(end)?;
        self.bytes.extend_from_slice(bytes);
        Ok(())
    }

    pub(super) fn list<T>(
        &mut self,
        values: &[T],
        mut write: impl FnMut(&mut Self, &T) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let count = u32::try_from(values.len()).map_err(|_| Error::Shape)?;
        self.push(&count.to_le_bytes())?;
        for value in values {
            write(self, value)?;
        }
        Ok(())
    }

    pub(super) fn finish(self) -> Vec<u8> {
        self.bytes
    }
}
