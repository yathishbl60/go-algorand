// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::implementation;

// Add a frozen reference only after establishing identical v1 constraints, schedules,
// encodings and backend parameters. Incompatible suites require versioned dispatch.
// Approval is compiled into the implementation, never supplied by an artifact/peer.
const COMPATIBLE_REFERENCES: [[u8; 64]; 0] = [];

/// Recognizes the current candidate or an explicitly supported frozen reference.
/// It is not suite approval; consensus must independently approve the registry/pins.
pub fn reference_supported(reference: &[u8; 64]) -> bool {
    reference == &implementation::ID || COMPATIBLE_REFERENCES.contains(reference)
}
