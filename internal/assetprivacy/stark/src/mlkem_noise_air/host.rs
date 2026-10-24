// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemNoiseAir, trace::output, witness::coefficients};

impl MlKemNoiseAir {
    /// Exact eta=2 PRF/CBD witness values without generating a trace matrix.
    pub fn values(&self, seed: [u8; 32]) -> [u16; 256] {
        coefficients(output(&self.message(seed)))
    }

    pub(super) fn message(&self, seed: [u8; 32]) -> [u8; 33] {
        let mut message = [0; 33];
        message[..32].copy_from_slice(&seed);
        message[32] = self.nonce;
        message
    }
}
