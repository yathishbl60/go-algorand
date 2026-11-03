// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Arc, DenseData, MaskedTrace, StreamedInputLimits, StreamedTree, Value32};
use p3_field::coset::TwoAdicMultiplicativeCoset;

/// Opaque real matrix storage; no private state cloning, Debug or serialization.
pub struct InputProverData<M> {
    pub(super) storage: Storage<M>,
}

pub(super) enum Storage<M> {
    Dense(DenseData<M>),
    Streamed(Streamed<M>),
}

pub(super) struct Streamed<M> {
    pub(super) matrices: Vec<M>,
    pub(super) coefficients: Vec<Arc<MaskedTrace<Value32>>>,
    pub(super) domain: TwoAdicMultiplicativeCoset<Value32>,
    pub(super) tree: StreamedTree,
    pub(super) limits: StreamedInputLimits,
}

impl<M> InputProverData<M> {
    pub(super) fn dense(data: DenseData<M>) -> Self {
        Self {
            storage: Storage::Dense(data),
        }
    }

    pub(super) fn streamed(
        matrices: Vec<M>,
        coefficients: Vec<Arc<MaskedTrace<Value32>>>,
        domain: TwoAdicMultiplicativeCoset<Value32>,
        tree: StreamedTree,
        limits: StreamedInputLimits,
    ) -> Self {
        Self {
            storage: Storage::Streamed(Streamed {
                matrices,
                coefficients,
                domain,
                tree,
                limits,
            }),
        }
    }
}
