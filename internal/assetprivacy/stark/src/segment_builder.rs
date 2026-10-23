// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::private_output_builder::SegmentWindow;
use core::ops::Range;
use p3_air::AirBuilder;

// Fixed compiled ranges only. Both child AIRs must declare zero public values.
pub(crate) struct SegmentBuilder<'a, AB: AirBuilder> {
    inner: &'a mut AB,
    columns: Range<usize>,
    preprocessing: SegmentWindow<AB::PreprocessedWindow>,
}

impl<'a, AB: AirBuilder> SegmentBuilder<'a, AB> {
    pub fn new(inner: &'a mut AB, columns: Range<usize>, preprocessed: Range<usize>) -> Self {
        let preprocessing = SegmentWindow::new(inner.preprocessed().clone(), preprocessed);
        Self {
            inner,
            columns,
            preprocessing,
        }
    }
}

impl<AB: AirBuilder> AirBuilder for SegmentBuilder<'_, AB> {
    type F = AB::F;
    type Expr = AB::Expr;
    type Var = AB::Var;
    type MainWindow = SegmentWindow<AB::MainWindow>;
    type PreprocessedWindow = SegmentWindow<AB::PreprocessedWindow>;
    type PublicVar = AB::PublicVar;
    type PeriodicVar = AB::PeriodicVar;

    fn main(&self) -> Self::MainWindow {
        SegmentWindow::new(self.inner.main(), self.columns.clone())
    }
    fn preprocessed(&self) -> &Self::PreprocessedWindow {
        &self.preprocessing
    }
    fn is_first_row(&self) -> Self::Expr {
        self.inner.is_first_row()
    }
    fn is_last_row(&self) -> Self::Expr {
        self.inner.is_last_row()
    }
    fn is_transition(&self) -> Self::Expr {
        self.inner.is_transition()
    }
    fn assert_zero<I: Into<Self::Expr>>(&mut self, x: I) {
        self.inner.assert_zero(x);
    }
    fn public_values(&self) -> &[Self::PublicVar] {
        &[]
    }
    fn periodic_values(&self) -> &[Self::PeriodicVar] {
        &[]
    }
}
