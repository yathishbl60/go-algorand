// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MAX_MESSAGE_BYTES, Sha3Air};
use crate::MIN_TRACE_ROWS;
use p3_keccak_air::NUM_ROUNDS;

impl Sha3Air {
    pub(super) fn with_digest_length(message_len: usize, digest_bytes: usize) -> Option<Self> {
        (message_len <= MAX_MESSAGE_BYTES).then_some(Self {
            message_len,
            digest_bytes,
            rate_bytes: 200 - 2 * digest_bytes,
            domain_suffix: 0x06,
            minimum_rows: MIN_TRACE_ROWS,
        })
    }

    pub(crate) fn shake256_128(message_len: usize) -> Option<Self> {
        (message_len <= MAX_MESSAGE_BYTES).then_some(Self {
            message_len,
            digest_bytes: 128,
            rate_bytes: 136,
            domain_suffix: 0x1f,
            minimum_rows: MIN_TRACE_ROWS,
        })
    }

    /// A parent fixes the larger common height; untrusted proof data cannot choose it.
    pub(crate) fn with_trace_rows(mut self, rows: usize) -> Option<Self> {
        if !rows.is_power_of_two() || rows < self.trace_rows() || rows > 8192 {
            return None;
        }
        self.minimum_rows = rows;
        Some(self)
    }

    pub(crate) fn rate_bytes(&self) -> usize {
        self.rate_bytes
    }

    pub fn digest_bytes(&self) -> usize {
        self.digest_bytes
    }

    pub fn message_bytes(&self) -> usize {
        self.message_len
    }

    /// Exact height produced by the local generator; not a proof-selected budget.
    pub fn trace_rows(&self) -> usize {
        self.minimum_rows
            .max((self.blocks() * NUM_ROUNDS).next_power_of_two())
    }
}
