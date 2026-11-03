// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    Config, Domain, E,
    coefficient_pcs::{Error, StripQuotient},
    fixture,
};
use p3_batch_stark::quotient_scheduler::{QuotientJob, QuotientScheduler};
use p3_field::PrimeCharacteristicRing;
use p3_uni_stark::StarkGenericConfig;
use std::cell::Cell;

pub fn run(job: &QuotientJob<'_, Config>) {
    let config = fixture::with_budget(1);
    let mut low_budget = copy(job);
    low_budget.pcs = config.pcs();
    rejected(&low_budget, true);
    let mut malformed = copy(job);
    malformed.quotient_domain = job.trace_domain;
    rejected(&malformed, false);
    let mut preprocessing = copy(job);
    preprocessing.preprocessed = Some(job.main);
    rejected(&preprocessing, false);
    wrong_length(job, false);
    wrong_length(job, true);
    callback_error(job);
}

fn wrong_length(job: &QuotientJob<'_, Config>, extra: bool) {
    let calls = Cell::new(0);
    let callback = |domain: Domain| {
        calls.set(calls.get() + 1);
        let rows = domain.size();
        let length = if extra { rows + 1 } else { rows - 1 };
        Ok(vec![E::ZERO; length])
    };
    let result = StripQuotient.evaluate(job, &callback);
    assert!(matches!(result, Err(Error::Shape)));
    assert_eq!(calls.get(), 1);
}

fn callback_error(job: &QuotientJob<'_, Config>) {
    let calls = Cell::new(0);
    let callback = |_| {
        calls.set(calls.get() + 1);
        Err(Error::Allocation)
    };
    let result = StripQuotient.evaluate(job, &callback);
    assert!(matches!(result, Err(Error::Allocation)));
    assert_eq!(calls.get(), 1);
}

fn rejected(job: &QuotientJob<'_, Config>, budget: bool) {
    let calls = Cell::new(0);
    let callback = |_| {
        calls.set(calls.get() + 1);
        Err(Error::Shape)
    };
    let result = StripQuotient.evaluate(job, &callback);
    assert!(result.is_err());
    if budget {
        assert!(matches!(result, Err(Error::Limits)));
    }
    assert_eq!(calls.get(), 0);
}

fn copy<'a>(job: &'a QuotientJob<'_, Config>) -> QuotientJob<'a, Config> {
    QuotientJob {
        pcs: job.pcs,
        instance_index: job.instance_index,
        trace_domain: job.trace_domain,
        quotient_domain: job.quotient_domain,
        layout: job.layout,
        main: job.main,
        permutation: job.permutation,
        preprocessed: job.preprocessed,
    }
}
