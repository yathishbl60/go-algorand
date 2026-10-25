// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::{HEADER, cursor::stark_header};

/// Local codec ceilings, not approved consensus work/fee parameters.
#[derive(Clone, Copy)]
pub struct BundleLimits {
    pub(super) wire: usize,
    pub(super) proof: usize,
    pub(super) allocation: usize,
}

impl BundleLimits {
    pub fn new(wire: usize, proof: usize, allocation: usize) -> Result<Self, Error> {
        if !(HEADER..=512 << 20).contains(&wire)
            || !(73..=128 << 20).contains(&proof)
            || proof > wire
        {
            return Err(Error::WireLimit);
        }
        if allocation == 0 || allocation > 512 << 20 {
            return Err(Error::MemoryLimit);
        }
        Ok(Self {
            wire,
            proof,
            allocation,
        })
    }

    pub(super) fn proof(&self, bytes: &[u8]) -> Result<(), Error> {
        if bytes.len() > self.proof {
            return Err(Error::WireLimit);
        }
        stark_header(bytes)
    }
}
