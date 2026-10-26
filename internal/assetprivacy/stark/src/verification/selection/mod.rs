// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Typed relation preparation and independently pinned public preprocessing keys.
mod bind;
mod check_setup;
mod dimensions;
mod domain;
mod entry;
mod layout;
mod main;
mod output;
mod prepared;
mod registry;
mod registry_codec;
mod registry_digest;
mod setup;

pub use domain::Domain;
pub use entry::Entry;
pub use layout::Layout;
pub use prepared::Prepared;
pub use registry::{Pin, Registry};
pub use registry_codec::encode_registry;
pub use registry_digest::registry_digest;
pub use setup::PublicSetup;
