// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Error, Fri, InputOpening, MultiProof, writer::Writer};
use crate::{Challenge, Value, types::ChallengeMmcs};
use p3_fri::CommitPhaseMultiStep;

impl Writer {
    fn rows(&mut self, rows: &[Vec<Value>]) -> Result<(), Error> {
        self.list(rows, |writer, row| writer.values(row))
    }

    fn multi_proof(&mut self, proof: &MultiProof) -> Result<(), Error> {
        self.list(&proof.0, |writer, rows| writer.rows(rows))?;
        self.list(&proof.1.sibling_hashes, Self::digest)
    }

    fn input_opening(&mut self, opening: &InputOpening) -> Result<(), Error> {
        self.list(&opening.opened_values, |writer, rows| writer.rows(rows))?;
        self.multi_proof(&opening.opening_proof)
    }

    fn phase(
        &mut self,
        phase: &CommitPhaseMultiStep<Challenge, ChallengeMmcs>,
    ) -> Result<(), Error> {
        self.list(&phase.sibling_values, |writer, row| writer.challenges(row))?;
        self.multi_proof(&phase.opening_proof)
    }

    pub(super) fn fri(&mut self, proof: &Fri) -> Result<(), Error> {
        self.zero(&proof.batch_pow_witness)?;
        self.list(&proof.commit_phase_commits, Self::cap)?;
        self.list(&proof.commit_pow_witnesses, Self::zero)?;
        self.list(&proof.input_openings, Self::input_opening)?;
        self.list(&proof.commit_phase_openings, Self::phase)?;
        self.challenges(&proof.final_poly)?;
        self.zero(&proof.query_pow_witness)
    }
}
