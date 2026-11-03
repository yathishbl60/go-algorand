// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private masked coefficients and caller-bounded natural-order evaluation tiles.
//! This is storage groundwork, not a PCS, memory bound or selected native profile.
mod coefficient_matrix;
mod coefficients;
mod evaluate;
mod input_mmcs;
mod masking;
mod streamed_tree;
mod strips;

pub use coefficient_matrix::CoefficientMatrix;
pub use coefficients::MaskedTrace;
pub use input_mmcs::{
    CoefficientInputData, InputError, InputMmcs, InputProverData, OwnedCoefficientMatrix,
    StreamedInputLimits,
};
pub use streamed_tree::{StreamedTree, TreeCap, TreeDigest};
pub use strips::{QUOTIENT_STRIP_COUNT, QuotientStrips};
