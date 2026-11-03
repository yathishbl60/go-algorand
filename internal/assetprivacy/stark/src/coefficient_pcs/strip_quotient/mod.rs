// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

mod budget;
mod preflight;
mod scatter;

use super::{Config, E, Error, V};
use crate::compact_storage::QUOTIENT_STRIP_COUNT;
use p3_batch_stark::quotient_scheduler::{DomainEvaluator, QuotientJob, QuotientScheduler};

/// Sequential exact coset strips; original selectors and stock quotient evaluator.
pub struct StripQuotient;

impl QuotientScheduler<Config> for StripQuotient {
    fn parallel_instances(&self) -> bool {
        false
    }

    fn evaluate(
        &self,
        job: &QuotientJob<'_, Config>,
        evaluate: &DomainEvaluator<'_, Config>,
    ) -> Result<Vec<E>, Error> {
        let strips = preflight::validate(job)?;
        let mut output = scatter::zeroes(strips.total_rows())?;
        for index in 0..QUOTIENT_STRIP_COUNT {
            let domain = strips.strip(index).ok_or(Error::Shape)?;
            let values = evaluate(domain)?;
            scatter::strip(&strips, index, &values, &mut output)?;
        }
        Ok(output)
    }
}
