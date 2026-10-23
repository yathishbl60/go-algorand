// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_air::{AirBuilder, WindowAccess};

pub(crate) struct PrefixBuilder<'a, AB> {
    pub inner: &'a mut AB,
    pub width: usize,
}

#[derive(Clone)]
pub(crate) struct PrefixWindow<W> {
    inner: W,
    width: usize,
}

impl<T, W: WindowAccess<T>> WindowAccess<T> for PrefixWindow<W> {
    fn current_slice(&self) -> &[T] {
        &self.inner.current_slice()[..self.width]
    }
    fn next_slice(&self) -> &[T] {
        &self.inner.next_slice()[..self.width]
    }
}

impl<AB: AirBuilder> AirBuilder for PrefixBuilder<'_, AB> {
    type F = AB::F;
    type Expr = AB::Expr;
    type Var = AB::Var;
    type PreprocessedWindow = AB::PreprocessedWindow;
    type MainWindow = PrefixWindow<AB::MainWindow>;
    type PublicVar = AB::PublicVar;
    type PeriodicVar = AB::PeriodicVar;

    fn main(&self) -> Self::MainWindow {
        PrefixWindow {
            inner: self.inner.main(),
            width: self.width,
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
    fn assert_zero<I: Into<Self::Expr>>(&mut self, x: I) {
        self.inner.assert_zero(x);
    }
    fn public_values(&self) -> &[Self::PublicVar] {
        self.inner.public_values()
    }
    fn periodic_values(&self) -> &[Self::PeriodicVar] {
        self.inner.periodic_values()
    }
}
