// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::private_output_builder::SegmentWindow;
use core::ops::Range;
use p3_air::AirBuilder;

// Compiled ranges only, never witness-selected. The parent must bind relayed
// main cells to the trusted program. Children must declare zero public/periodic
// values and no next-row preprocessing reads; this adapter adds no such binding.
pub(crate) struct RelayBuilder<'a, AB: AirBuilder> {
    inner: &'a mut AB,
    columns: Range<usize>,
    preprocessing: SegmentWindow<AB::MainWindow>,
}

impl<'a, AB: AirBuilder> RelayBuilder<'a, AB> {
    pub fn new(inner: &'a mut AB, columns: Range<usize>, relayed: Range<usize>) -> Self {
        let preprocessing = SegmentWindow::new(inner.main().clone(), relayed);
        Self {
            inner,
            columns,
            preprocessing,
        }
    }
}

impl<AB: AirBuilder> AirBuilder for RelayBuilder<'_, AB> {
    type F = AB::F;
    type Expr = AB::Expr;
    type Var = AB::Var;
    type MainWindow = SegmentWindow<AB::MainWindow>;
    type PreprocessedWindow = SegmentWindow<AB::MainWindow>;
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
    fn assert_zero<I: Into<Self::Expr>>(&mut self, value: I) {
        self.inner.assert_zero(value);
    }
    fn public_values(&self) -> &[Self::PublicVar] {
        &[]
    }
    fn periodic_values(&self) -> &[Self::PeriodicVar] {
        &[]
    }
}
