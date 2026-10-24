// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::EnvelopeContext;

/// Public statement fields. Role is recipient=1, auditor=2, controller=3.
pub struct EnvelopeFields {
    pub network: [u8; 32],
    pub asset: u64,
    pub policy: [u8; 64],
    pub role: u8,
    pub nonce: [u8; 32],
    pub output: u32,
    pub commitment: [u8; 64],
}

impl EnvelopeContext {
    pub fn new(fields: EnvelopeFields) -> Option<Self> {
        let mut bytes = [0; 248];
        bytes[..42].copy_from_slice(b"Algorand/AssetPrivacy/Candidate1/Envelope/");
        bytes[42] = 1;
        bytes[43..75].copy_from_slice(&fields.network);
        bytes[75..83].copy_from_slice(&fields.asset.to_be_bytes());
        bytes[83..147].copy_from_slice(&fields.policy);
        bytes[147] = fields.role;
        bytes[148..180].copy_from_slice(&fields.nonce);
        bytes[180..184].copy_from_slice(&fields.output.to_be_bytes());
        bytes[184..].copy_from_slice(&fields.commitment);
        Self::parse(bytes)
    }
}
