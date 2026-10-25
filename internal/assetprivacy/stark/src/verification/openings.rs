// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Error, reader::Reader, shape::Shape};
use crate::Challenge;
use p3_uni_stark::{OpenedValues, PreprocessedOpenedValues};

impl Reader<'_> {
    fn row(&mut self, width: usize) -> Result<Vec<Challenge>, Error> {
        self.fixed(width, 40, Self::challenge)
    }

    pub(super) fn openings(&mut self, shape: Shape) -> Result<OpenedValues<Challenge>, Error> {
        let trace_local = self.row(shape.width)?;
        let trace_next = shape.main_next.then(|| self.row(shape.width)).transpose()?;
        let local = self.row(shape.preprocessed)?;
        let next = shape
            .preprocessed_next
            .then(|| self.row(shape.preprocessed))
            .transpose()?;
        let quotient_chunks = self.list(204, |reader| reader.row(5))?;
        Ok(OpenedValues {
            trace_local,
            trace_next,
            preprocessed: Some(PreprocessedOpenedValues { local, next }),
            quotient_chunks,
            random: Some(self.row(5)?),
        })
    }

    fn point(&mut self) -> Result<Vec<Challenge>, Error> {
        self.challenges()
    }

    fn matrix(&mut self) -> Result<Vec<Vec<Challenge>>, Error> {
        self.list(4, Self::point)
    }

    fn round(&mut self) -> Result<Vec<Vec<Vec<Challenge>>>, Error> {
        self.list(4, Self::matrix)
    }

    pub(super) fn random_openings(&mut self) -> Result<p3_commit::OpenedValues<Challenge>, Error> {
        self.list(4, Self::round)
    }
}
