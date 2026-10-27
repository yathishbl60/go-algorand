// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Thin unsafe C boundary. The core verifier crate continues to forbid unsafe code.
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(feature = "native-privacy")]
mod exports;
#[cfg(feature = "native-privacy")]
mod inputs;
