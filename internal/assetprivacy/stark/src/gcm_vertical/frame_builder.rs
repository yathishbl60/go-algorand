// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::frame::Frame;
use crate::private_output_builder::SegmentWindow;
use core::ops::Range;
use p3_air::{AirBuilder, WindowAccess};

pub(crate) struct FrameBuilder<'a, AB: AirBuilder> {
    inner: &'a mut AB,
    columns: Range<usize>,
    public: Range<usize>,
    preprocessing: SegmentWindow<AB::PreprocessedWindow>,
    flags: [AB::Var; 4],
}

impl<'a, AB: AirBuilder> FrameBuilder<'a, AB> {
    pub fn new(inner: &'a mut AB, frame: Frame) -> Self {
        let flags = inner.preprocessed().current_slice()[frame.flags..frame.flags + 4]
            .try_into()
            .expect("four trusted frame flags");
        let preprocessing = SegmentWindow::new(inner.preprocessed().clone(), frame.preprocessing);
        Self {
            inner,
            columns: frame.columns,
            public: frame.public,
            preprocessing,
            flags,
        }
    }
}

impl<AB: AirBuilder> AirBuilder for FrameBuilder<'_, AB> {
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
        self.flags[1].into()
    }
    fn is_last_row(&self) -> Self::Expr {
        self.flags[2].into()
    }
    fn is_transition(&self) -> Self::Expr {
        self.flags[3].into()
    }
    fn assert_zero<I: Into<Self::Expr>>(&mut self, x: I) {
        self.inner.assert_zero(self.flags[0].into() * x.into());
    }
    fn public_values(&self) -> &[Self::PublicVar] {
        &self.inner.public_values()[self.public.clone()]
    }
    fn periodic_values(&self) -> &[Self::PeriodicVar] {
        &[]
    }
}
