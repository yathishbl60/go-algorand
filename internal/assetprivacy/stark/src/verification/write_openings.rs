// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Error, writer::Writer};
use crate::Challenge;
use p3_uni_stark::OpenedValues;

impl Writer {
    pub(super) fn openings(&mut self, values: &OpenedValues<Challenge>) -> Result<(), Error> {
        self.challenges(&values.trace_local)?;
        if let Some(next) = &values.trace_next {
            self.challenges(next)?;
        }
        let pre = values.preprocessed.as_ref().ok_or(Error::Shape)?;
        self.challenges(&pre.local)?;
        if let Some(next) = &pre.next {
            self.challenges(next)?;
        }
        self.list(&values.quotient_chunks, |writer, row| {
            writer.challenges(row)
        })?;
        self.challenges(values.random.as_ref().ok_or(Error::Shape)?)
    }

    fn points(&mut self, points: &[Vec<Challenge>]) -> Result<(), Error> {
        self.list(points, |writer, row| writer.challenges(row))
    }

    fn matrices(&mut self, matrices: &[Vec<Vec<Challenge>>]) -> Result<(), Error> {
        self.list(matrices, |writer, points| writer.points(points))
    }

    pub(super) fn random_openings(
        &mut self,
        values: &p3_commit::OpenedValues<Challenge>,
    ) -> Result<(), Error> {
        self.list(values, |writer, matrices| writer.matrices(matrices))
    }
}
