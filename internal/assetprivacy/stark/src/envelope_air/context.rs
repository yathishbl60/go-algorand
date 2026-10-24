// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::note_air::NoteCommitmentAir;

const DOMAIN: &[u8] = b"Algorand/AssetPrivacy/Candidate1/Envelope/";

/// Canonical public context, constructed from the verifier's finalized statement.
#[derive(Clone, Copy)]
pub struct EnvelopeContext {
    bytes: [u8; 248],
}

impl EnvelopeContext {
    pub fn parse(bytes: [u8; 248]) -> Option<Self> {
        if &bytes[..42] != DOMAIN || bytes[42] != 1 || !(1..=3).contains(&bytes[147]) {
            return None;
        }
        for range in [43..75, 75..83, 83..147, 148..180, 184..248] {
            if bytes[range].iter().all(|&value| value == 0) {
                return None;
            }
        }
        Some(Self { bytes })
    }

    pub fn bytes(&self) -> &[u8; 248] {
        &self.bytes
    }

    pub(crate) fn encryption_bytes(&self, epoch: u64) -> [u8; 256] {
        let mut bytes = [0; 256];
        bytes[..147].copy_from_slice(&self.bytes[..147]);
        bytes[147..155].copy_from_slice(&epoch.to_be_bytes());
        bytes[155..].copy_from_slice(&self.bytes[147..]);
        bytes
    }

    pub(crate) fn note(&self) -> Option<NoteCommitmentAir> {
        NoteCommitmentAir::new(
            self.bytes[43..75].try_into().ok()?,
            u64::from_be_bytes(self.bytes[75..83].try_into().ok()?),
            self.bytes[83..147].try_into().ok()?,
        )
    }

    pub(crate) fn commitment(&self) -> [u8; 64] {
        self.bytes[184..]
            .try_into()
            .expect("fixed context commitment")
    }
}
