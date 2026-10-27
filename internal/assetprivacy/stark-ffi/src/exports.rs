// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::panic::{AssertUnwindSafe, catch_unwind};

#[unsafe(no_mangle)]
pub extern "C" fn nap_privacy_abi() -> u32 {
    1
}

/// # Safety
/// `source` covers `source_len` initialized immutable bytes for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nap_privacy_source_v1(source: *const u8, source_len: usize) -> u32 {
    let checked = catch_unwind(AssertUnwindSafe(|| unsafe {
        super::inputs::source(source, source_len)
    }));
    outcome(checked)
}

/// # Safety
/// Both ranges must be valid, initialized, immutable byte buffers for the entire call.
/// The caller owns/pins them; Rust neither retains nor frees either range.
/// Request bytes are trusted native state, never transaction/peer-selected inputs.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nap_privacy_verify_v1(
    request: *const u8,
    request_len: usize,
    bundle: *const u8,
    bundle_len: usize,
) -> u32 {
    let checked = catch_unwind(AssertUnwindSafe(|| unsafe {
        super::inputs::verify(request, request_len, bundle, bundle_len)
    }));
    outcome(checked)
}

fn outcome(checked: std::thread::Result<Result<(), u32>>) -> u32 {
    match checked {
        Ok(Ok(())) => 0,
        Ok(Err(status)) => status,
        Err(_) => 4,
    }
}
