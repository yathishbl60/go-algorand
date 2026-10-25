// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    gcm_vertical::{frame::Frame, frame_builder::FrameBuilder},
    private_output_builder::SegmentWindow,
};
use core::ops::Range;
use p3_air::AirBuilder;

// Public entries are copied solely from trusted indices in the parent's statement.
// This never routes private witness columns or proof-supplied indices into public IO.
pub(super) struct MappedBuilder<'a, AB: AirBuilder> {
    inner: FrameBuilder<'a, AB>,
    public: Vec<AB::PublicVar>,
}

impl<'a, AB: AirBuilder> MappedBuilder<'a, AB> {
    pub(super) fn new(inner: &'a mut AB, frame: Frame, ranges: &[Range<usize>]) -> Self {
        let public = ranges
            .iter()
            .flat_map(|range| inner.public_values()[range.clone()].iter().copied())
            .collect();
        Self {
            inner: FrameBuilder::new(inner, frame),
            public,
        }
    }
}

impl<AB: AirBuilder> AirBuilder for MappedBuilder<'_, AB> {
    type F = AB::F;
    type Expr = AB::Expr;
    type Var = AB::Var;
    type MainWindow = SegmentWindow<AB::MainWindow>;
    type PreprocessedWindow = SegmentWindow<AB::PreprocessedWindow>;
    type PublicVar = AB::PublicVar;
    type PeriodicVar = AB::PeriodicVar;

    fn main(&self) -> Self::MainWindow {
        self.inner.main()
    }
    fn preprocessed(&self) -> &Self::PreprocessedWindow {
        self.inner.preprocessed()
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
        &self.public
    }
    fn periodic_values(&self) -> &[Self::PeriodicVar] {
        self.inner.periodic_values()
    }
}
