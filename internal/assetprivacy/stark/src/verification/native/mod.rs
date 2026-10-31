// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Trusted native caller request parsing and concrete verification dispatch.
//! Requests are not transaction/witness data and cannot establish their own trust.
mod binding;
mod build;
mod confidential;
mod confidential_spend;
mod cursor;
mod error;
mod expected;
mod expected_main;
mod header;
mod header_fields;
mod main;
mod main_build;
mod output;
mod owner_registration;
mod request;
mod shield;
mod slot;
mod target;
mod verify;
mod viewing_registration;

pub use error::Failure;
pub use verify::verify_request;
pub const MAX_REQUEST_BYTES: usize = 16248;
pub const MAX_BUNDLE_BYTES: usize = 512 << 20;
const HEADER: usize = 1824;
