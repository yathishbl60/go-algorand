// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Error;

/// Local admission ceilings, not approved consensus fee or work parameters.
#[derive(Clone, Copy)]
pub struct DecodeLimits {
    pub(super) wire: usize,
    pub(super) allocation: usize,
}

impl DecodeLimits {
    /// The caller must select smaller deployment limits from measured resources.
    pub fn new(wire_bytes: usize, allocation_bytes: usize) -> Result<Self, Error> {
        if wire_bytes == 0 || wire_bytes > 128 << 20 {
            return Err(Error::WireLimit);
        }
        if allocation_bytes == 0 || allocation_bytes > 512 << 20 {
            return Err(Error::MemoryLimit);
        }
        Ok(Self {
            wire: wire_bytes,
            allocation: allocation_bytes,
        })
    }
}
