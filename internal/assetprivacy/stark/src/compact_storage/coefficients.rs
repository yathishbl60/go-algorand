// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::masking::{mask, valid_trace};
use p3_dft::TwoAdicSubgroupDft;
use p3_field::{TwoAdicField, coset::TwoAdicMultiplicativeCoset};
use p3_matrix::{Matrix, dense::RowMajorMatrix};
use rand::{
    CryptoRng,
    distr::{Distribution, StandardUniform},
};

/// Secret, uncommitted polynomial storage; contains masks and private trace data.
/// No Clone/Debug/serialization is provided. Accessors expose PRIVATE material.
pub struct MaskedTrace<F: TwoAdicField> {
    pub(super) coefficients: RowMajorMatrix<F>,
    trace_rows: usize,
    relation_width: usize,
}

impl<F: TwoAdicField> MaskedTrace<F> {
    /// Interleave stock random rows/codewords and interpolate on shift*H_(2N).
    /// `domain` is the ORIGINAL N-row trace domain. Use fresh secret RNG entropy.
    /// This performs no commitment and enforces no one-opening disclosure budget.
    pub fn from_trace<D: TwoAdicSubgroupDft<F>, R: CryptoRng>(
        trace: RowMajorMatrix<F>,
        domain: TwoAdicMultiplicativeCoset<F>,
        dft: &D,
        rng: &mut R,
    ) -> Option<Self>
    where
        StandardUniform: Distribution<F>,
    {
        if !valid_trace(&trace, domain) {
            return None;
        }
        let shape = (trace.height(), trace.width());
        let randomized = mask(trace, rng)?;
        let coefficients = dft.coset_idft_batch(randomized, domain.shift());
        Some(Self::from_coefficients(coefficients, shape))
    }

    fn from_coefficients(coefficients: RowMajorMatrix<F>, shape: (usize, usize)) -> Self {
        Self {
            coefficients,
            trace_rows: shape.0,
            relation_width: shape.1,
        }
    }

    pub fn trace_rows(&self) -> usize {
        self.trace_rows
    }

    pub fn relation_width(&self) -> usize {
        self.relation_width
    }

    pub fn width(&self) -> usize {
        self.coefficients.width()
    }

    pub fn coefficient_rows(&self) -> usize {
        self.coefficients.height()
    }

    /// Private coefficients in natural degree order, including ten random columns.
    pub fn coefficients(&self) -> &RowMajorMatrix<F> {
        &self.coefficients
    }
}
