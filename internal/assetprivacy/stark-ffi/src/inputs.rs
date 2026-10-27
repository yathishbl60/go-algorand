// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use algorand_asset_privacy_stark::verification::{Error, native};
use std::slice;

/// # Safety
/// The caller guarantees an initialized immutable range covering `length` bytes.
pub(super) unsafe fn source(pointer: *const u8, length: usize) -> Result<(), u32> {
    if length != 64 {
        return Err(1);
    }
    let expected = unsafe { borrow(pointer, length, 64)? };
    let expected = expected.try_into().map_err(|_| 1u32)?;
    if !algorand_asset_privacy_stark::verification::selection::reference_supported(expected) {
        return Err(2);
    }
    Ok(())
}

/// # Safety
/// The caller guarantees initialized immutable buffers covering the provided lengths.
pub(super) unsafe fn verify(
    request: *const u8,
    request_len: usize,
    bundle: *const u8,
    bundle_len: usize,
) -> Result<(), u32> {
    let request = unsafe { borrow(request, request_len, native::MAX_REQUEST_BYTES)? };
    let bundle = unsafe { borrow(bundle, bundle_len, native::MAX_BUNDLE_BYTES)? };
    native::verify_request(request, bundle).map_err(status)
}

/// # Safety
/// `pointer` must cover `length` initialized immutable bytes for the returned borrow.
unsafe fn borrow<'a>(pointer: *const u8, length: usize, maximum: usize) -> Result<&'a [u8], u32> {
    if pointer.is_null() || length == 0 || length > maximum || length > isize::MAX as usize {
        return Err(1);
    }
    // SAFETY: the trusted foreign caller supplies validity/immutability; bounds checked above.
    Ok(unsafe { slice::from_raw_parts(pointer, length) })
}

fn status(failure: native::Failure) -> u32 {
    match failure {
        native::Failure::Request(Error::AllocationFailure)
        | native::Failure::Proof(Error::AllocationFailure) => 5,
        native::Failure::Request(Error::BackendPanic)
        | native::Failure::Proof(Error::BackendPanic) => 4,
        native::Failure::Request(Error::Artifact) => 2,
        native::Failure::Request(_) => 1,
        native::Failure::Proof(_) => 3,
    }
}
