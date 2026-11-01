// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Challenge32;
use p3_field::extension::HasFrobenius;

/// Rejects every proper subfield of the degree-ten extension.
/// Private-pad basis challenges need this check; non-base alone is insufficient.
/// This does not sample a challenge or establish a complete hiding protocol.
pub fn has_full_extension_degree(value: Challenge32) -> bool {
    value.repeated_frobenius(2) != value && value.repeated_frobenius(5) != value
}
