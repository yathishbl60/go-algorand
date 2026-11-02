// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

mod evaluation;
mod fixture;
pub mod masking;
pub mod quotient;
mod tiles;

use algorand_asset_privacy_stark::compact::{Challenge32 as E, Config, Value32 as V};
use p3_field::coset::TwoAdicMultiplicativeCoset as Domain;
use p3_matrix::dense::RowMajorMatrix;

type Matrix = RowMajorMatrix<V>;
