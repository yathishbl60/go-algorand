// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private rho-to-polynomial ML-KEM matrix relation with shared prefix wires.
//! Bounded prefix exhaustion fails; complete encapsulation remains separate.
mod layout;
mod schedule;
mod trace;

use crate::{
    Value, mlkem_sample_air::MlKemSampleAir, segment_builder::SegmentBuilder,
    shake128_matrix_air::Shake128MatrixAir,
};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_keccak_air::NUM_KECCAK_COLS;
use p3_matrix::dense::RowMajorMatrix;

#[derive(Clone, Copy)]
pub struct MlKemMatrixAir {
    xof: Shake128MatrixAir,
    sample: MlKemSampleAir,
}

impl MlKemMatrixAir {
    /// Candidate protocol admission fixes all matrix entries to this bound.
    pub fn candidate(first: u8, second: u8) -> Option<Self> {
        Self::new(first, second, crate::shake128_matrix_air::MAX_RATE_BLOCKS)
    }
    pub fn new(first: u8, second: u8, blocks: usize) -> Option<Self> {
        let sample = MlKemSampleAir::new(blocks)?;
        let xof = Shake128MatrixAir::new(first, second, blocks)?.with_rows(sample.trace_rows())?;
        Some(Self { xof, sample })
    }
    pub fn trace_rows(&self) -> usize {
        self.sample.trace_rows()
    }
    pub fn prefix_bytes(&self) -> usize {
        self.sample.prefix_bytes()
    }
    pub(super) const fn sampler_start(&self) -> usize {
        NUM_KECCAK_COLS + 256
    }
}

impl BaseAir<Value> for MlKemMatrixAir {
    fn width(&self) -> usize {
        self.sampler_start() + self.sample.width()
    }
    fn preprocessed_width(&self) -> usize {
        self.xof.preprocessed_width() + self.sample.preprocessed_width()
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        self.schedule()
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for MlKemMatrixAir {
    fn eval(&self, builder: &mut AB) {
        let xof_width = self.xof.trace_width();
        let first = self.xof.preprocessed_width();
        self.xof
            .eval(&mut SegmentBuilder::new(builder, 0..xof_width, 0..first));
        let columns = self.sampler_start()..self.width();
        let preprocessed = first..self.preprocessed_width();
        self.sample
            .eval(&mut SegmentBuilder::new(builder, columns, preprocessed));
    }
}
