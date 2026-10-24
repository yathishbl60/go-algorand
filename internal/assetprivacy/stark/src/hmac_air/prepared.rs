// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::trace::padded_message;
use sha3::{Digest, Sha3_512};

pub(super) struct Prepared {
    pub inner: Vec<u8>,
    pub outer: Vec<u8>,
    pub inner_digest: [u8; 64],
    pub output: [u8; 64],
}

impl Prepared {
    pub fn new(key: &[u8; 64], data: &[u8]) -> Self {
        let inner = padded_message(key, 0x36, data);
        let inner_digest: [u8; 64] = Sha3_512::digest(&inner).into();
        let outer = padded_message(key, 0x5c, &inner_digest);
        let output = Sha3_512::digest(&outer).into();
        Self {
            inner,
            outer,
            inner_digest,
            output,
        }
    }
}

pub(crate) fn derive_output(key: &[u8; 64], data: &[u8]) -> [u8; 64] {
    Prepared::new(key, data).output
}
