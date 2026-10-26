// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub(super) const ID: [u8; 64] = include!(concat!(env!("OUT_DIR"), "/nap_source_id.rs"));

/// Reproducible source/dependency identity, not an audit or proof of correct behavior.
pub fn implementation_id() -> [u8; 64] {
    ID
}
