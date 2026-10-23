// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::ops::Range;
use p3_air::{AirBuilder, WindowAccess};

pub(super) struct SegmentBuilder<'a, AB: AirBuilder> {
    pub inner: &'a mut AB,
    pub window: AB::MainWindow,
    pub columns: Range<usize>,
    pub public: Range<usize>,
}

#[derive(Clone)]
pub(super) struct SegmentWindow<W> {
    inner: W,
    columns: Range<usize>,
}

impl<T, W: WindowAccess<T>> WindowAccess<T> for SegmentWindow<W> {
    fn current_slice(&self) -> &[T] {
        &self.inner.current_slice()[self.columns.clone()]
    }
    fn next_slice(&self) -> &[T] {
        &self.inner.next_slice()[self.columns.clone()]
    }
}

impl<AB: AirBuilder> AirBuilder for SegmentBuilder<'_, AB> {
    type F = AB::F;
    type Expr = AB::Expr;
    type Var = AB::Var;
    type MainWindow = SegmentWindow<AB::MainWindow>;
    type PreprocessedWindow = AB::PreprocessedWindow;
    type PublicVar = AB::PublicVar;
    type PeriodicVar = AB::PeriodicVar;

    fn main(&self) -> Self::MainWindow {
        SegmentWindow {
            inner: self.window.clone(),
            columns: self.columns.clone(),
        }
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
        &self.inner.public_values()[self.public.clone()]
    }
    fn periodic_values(&self) -> &[Self::PeriodicVar] {
        self.inner.periodic_values()
    }
}
