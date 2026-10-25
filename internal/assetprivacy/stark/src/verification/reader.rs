// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DecodeLimits, Error};

pub(super) struct Reader<'a> {
    remaining: &'a [u8],
    allocation: usize,
}

impl<'a> Reader<'a> {
    pub(super) fn new(bytes: &'a [u8], limits: DecodeLimits) -> Result<Self, Error> {
        if bytes.len() > limits.wire {
            return Err(Error::WireLimit);
        }
        Ok(Self {
            remaining: bytes,
            allocation: limits.allocation,
        })
    }

    pub(super) fn array<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        let (value, rest) = self.remaining.split_at_checked(N).ok_or(Error::Truncated)?;
        let result = value.try_into().map_err(|_| Error::Truncated)?;
        self.remaining = rest;
        Ok(result)
    }

    pub(super) fn byte(&mut self) -> Result<u8, Error> {
        Ok(self.array::<1>()?[0])
    }

    pub(super) fn word(&mut self) -> Result<u64, Error> {
        Ok(u64::from_le_bytes(self.array()?))
    }

    fn reserve<T>(&mut self, count: usize, minimum: usize) -> Result<Vec<T>, Error> {
        if count > self.remaining.len() / minimum {
            return Err(Error::Truncated);
        }
        self.allocate(count)
    }

    pub(super) fn allocate<T>(&mut self, count: usize) -> Result<Vec<T>, Error> {
        let bytes = count
            .checked_mul(core::mem::size_of::<T>())
            .ok_or(Error::MemoryLimit)?;
        self.allocation = self
            .allocation
            .checked_sub(bytes)
            .ok_or(Error::MemoryLimit)?;
        let mut values = Vec::new();
        values
            .try_reserve_exact(count)
            .map_err(|_| Error::MemoryLimit)?;
        Ok(values)
    }

    pub(super) fn list<T>(
        &mut self,
        minimum: usize,
        mut read: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        let count = u32::from_le_bytes(self.array()?) as usize;
        let mut values = self.reserve(count, minimum)?;
        for _ in 0..count {
            values.push(read(self)?);
        }
        Ok(values)
    }

    pub(super) fn finish(self) -> Result<(), Error> {
        if !self.remaining.is_empty() {
            return Err(Error::NonCanonical);
        }
        Ok(())
    }

    pub(super) fn fixed<T>(
        &mut self,
        count: usize,
        minimum: usize,
        mut read: impl FnMut(&mut Self) -> Result<T, Error>,
    ) -> Result<Vec<T>, Error> {
        if u32::from_le_bytes(self.array()?) as usize != count {
            return Err(Error::Shape);
        }
        let mut values = self.reserve(count, minimum)?;
        for _ in 0..count {
            values.push(read(self)?);
        }
        Ok(values)
    }
}
