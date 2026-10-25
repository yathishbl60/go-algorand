// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Error, Fri, InputOpening, MultiProof, Paths, reader::Reader};
use crate::{Challenge, types::ChallengeMmcs};
use p3_fri::CommitPhaseMultiStep;

impl Reader<'_> {
    fn rows(&mut self) -> Result<Vec<Vec<crate::Value>>, Error> {
        self.list(4, Self::values)
    }

    fn multi_proof(&mut self) -> Result<MultiProof, Error> {
        let salts = self.list(4, Self::rows)?;
        let sibling_hashes = self.list(64, Self::digest)?;
        Ok((salts, Paths { sibling_hashes }))
    }

    fn input_opening(&mut self) -> Result<InputOpening, Error> {
        Ok(InputOpening {
            opened_values: self.list(4, Self::rows)?,
            opening_proof: self.multi_proof()?,
        })
    }

    fn phase(&mut self) -> Result<CommitPhaseMultiStep<Challenge, ChallengeMmcs>, Error> {
        Ok(CommitPhaseMultiStep {
            sibling_values: self.list(4, Self::challenges)?,
            opening_proof: self.multi_proof()?,
        })
    }

    pub(super) fn fri(&mut self) -> Result<Fri, Error> {
        Ok(Fri {
            batch_pow_witness: self.zero()?,
            commit_phase_commits: self.list(64, Self::cap)?,
            commit_pow_witnesses: self.list(8, Self::zero)?,
            input_openings: self.list(8, Self::input_opening)?,
            commit_phase_openings: self.list(8, Self::phase)?,
            final_poly: self.challenges()?,
            query_pow_witness: self.zero()?,
        })
    }
}
