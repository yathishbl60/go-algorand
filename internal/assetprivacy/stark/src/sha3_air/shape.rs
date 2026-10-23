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
        })
    }

    pub(crate) fn rate_bytes(&self) -> usize {
        200 - 2 * self.digest_bytes
    }

    pub fn digest_bytes(&self) -> usize {
        self.digest_bytes
    }

    /// Exact height produced by the local generator; not a proof-selected budget.
    pub fn trace_rows(&self) -> usize {
        MIN_TRACE_ROWS.max((self.blocks() * NUM_ROUNDS).next_power_of_two())
    }
}
