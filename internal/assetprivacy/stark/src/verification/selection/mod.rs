// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Typed relation preparation and independently pinned public preprocessing keys.
mod bind;
mod check_setup;
mod confidential;
mod confidential_air;
mod dimensions;
mod dimensions_other;
mod domain;
mod entry;
mod layout;
mod main;
mod output;
mod owner_registration;
mod prepared;
mod reference;
mod registry;
mod registry_codec;
mod registry_digest;
mod setup;
mod shield;
mod viewing_registration;

pub use domain::Domain;
pub use entry::Entry;
pub use layout::Layout;
pub use prepared::Prepared;
pub use reference::reference_supported;
pub use registry::{Pin, Registry};
pub use registry_codec::encode_registry;
pub use registry_digest::registry_digest;
pub use setup::PublicSetup;
