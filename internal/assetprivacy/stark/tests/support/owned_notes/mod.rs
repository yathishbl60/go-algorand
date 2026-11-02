// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

mod context;
mod fixture;
pub mod malformed;
pub mod pipeline;
pub mod private_links;
mod splicing;

use algorand_asset_privacy_stark::{compact::Value32, compact_owned_notes_air::OwnedNotesAir};
use p3_matrix::dense::RowMajorMatrix;

type Air = OwnedNotesAir<Value32>;
type Trace = RowMajorMatrix<Value32>;
