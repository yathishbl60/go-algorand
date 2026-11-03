// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    Config, Domain, E,
    coefficient_pcs::{Error, StripQuotient},
    guards,
};
use p3_batch_stark::quotient_scheduler::{DomainEvaluator, QuotientJob, QuotientScheduler};
use std::cell::Cell;

pub struct CheckedStrips {
    pub differential: bool,
}

impl QuotientScheduler<Config> for CheckedStrips {
    fn parallel_instances(&self) -> bool {
        false
    }

    fn evaluate(
        &self,
        job: &QuotientJob<'_, Config>,
        evaluate: &DomainEvaluator<'_, Config>,
    ) -> Result<Vec<E>, Error> {
        metadata(job);
        guards::run(job);
        let calls = Cell::new(0);
        let strip_evaluator = |domain: Domain| {
            assert_eq!(domain.size(), job.trace_domain.size() * 2);
            calls.set(calls.get() + 1);
            evaluate(domain)
        };
        let result = StripQuotient.evaluate(job, &strip_evaluator)?;
        assert_eq!(calls.get(), 8);
        if self.differential {
            assert_eq!(result, evaluate(job.quotient_domain)?);
        }
        Ok(result)
    }
}

fn metadata(job: &QuotientJob<'_, Config>) {
    assert!(!StripQuotient.parallel_instances());
    assert_eq!(job.layout.main_width, 1988);
    assert_eq!(job.layout.num_periodic_columns, 57);
    assert!(job.permutation.is_some());
    assert!(job.preprocessed.is_none());
    assert_eq!(job.quotient_domain.size(), 16 * job.trace_domain.size());
}
