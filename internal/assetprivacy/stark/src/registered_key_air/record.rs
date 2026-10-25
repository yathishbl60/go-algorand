// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Value, spend_key_air::SpendKeyAir, view_key_air::ViewKeyAir};
use p3_air::BaseAir;

#[derive(Clone)]
pub(super) enum RecordAir {
    View(ViewKeyAir),
    Spend(SpendKeyAir),
}

impl RecordAir {
    pub fn width(&self) -> usize {
        match self {
            Self::View(air) => air.width(),
            Self::Spend(air) => air.width(),
        }
    }
    pub fn rows(&self) -> usize {
        match self {
            Self::View(_) => crate::view_key_air::ROWS,
            Self::Spend(_) => crate::spend_key_air::ROWS,
        }
    }
    pub fn pre_width(&self) -> usize {
        match self {
            Self::View(air) => air.preprocessed_width(),
            Self::Spend(air) => air.preprocessed_width(),
        }
    }
    pub fn preprocessing(&self) -> Option<p3_matrix::dense::RowMajorMatrix<Value>> {
        match self {
            Self::View(air) => air.preprocessed_trace(),
            Self::Spend(air) => air.preprocessed_trace(),
        }
    }
}
