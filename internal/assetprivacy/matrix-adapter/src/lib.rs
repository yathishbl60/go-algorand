// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Checked row access bridge for P3's required unchecked trait method signature.
#![forbid(unsafe_op_in_unsafe_fn)]

mod matrix;
mod rows;

use core::marker::PhantomData;
pub use rows::CheckedRows;

/// A Matrix wrapper with fully checked row bodies and no private data copying.
pub struct SafeMatrix<M, T> {
    inner: M,
    value: PhantomData<T>,
}

impl<M: CheckedRows<T>, T: Clone + Send + Sync> SafeMatrix<M, T> {
    pub fn new(inner: M) -> Option<Self> {
        inner.width().checked_mul(inner.height())?;
        Some(Self {
            inner,
            value: PhantomData,
        })
    }

    pub fn inner(&self) -> &M {
        &self.inner
    }

    pub fn into_inner(self) -> M {
        self.inner
    }
}
