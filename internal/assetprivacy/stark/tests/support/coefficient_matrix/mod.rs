// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

mod fixture;
mod gather;
mod generic;
pub mod malformed;
pub mod modes;

use algorand_asset_privacy_stark::{compact::Value32, compact_storage::MaskedTrace};
use p3_field::coset::TwoAdicMultiplicativeCoset;
use p3_matrix::dense::RowMajorMatrix;

type V = Value32;
type Dense = RowMajorMatrix<V>;
type Domain = TwoAdicMultiplicativeCoset<V>;
type Mask = MaskedTrace<V>;
