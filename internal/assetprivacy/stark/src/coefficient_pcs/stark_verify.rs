// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Challenger, CoefficientPcs, Domain, E, V};
use p3_commit::{PeriodicColumns, PeriodicLdeTable, UnivariateStarkPcs};

pub(super) fn periodics(
    pcs: &CoefficientPcs,
    columns: &[Vec<V>],
    trace: Domain,
    quotient: Domain,
) -> PeriodicLdeTable<V> {
    capacity(pcs, columns, trace, quotient);
    UnivariateStarkPcs::<E, Challenger>::build_periodic_lde_table(
        &pcs.stock, columns, trace, quotient,
    )
}

fn capacity(pcs: &CoefficientPcs, columns: &[Vec<V>], trace: Domain, quotient: Domain) {
    let declared = PeriodicColumns::new(columns, trace.size()).expect("invalid periodic columns");
    assert!(
        quotient.size() >= trace.size(),
        "invalid periodic quotient domain"
    );
    let rows = declared
        .max_period()
        .unwrap_or(0)
        .checked_mul(quotient.size() / trace.size())
        .expect("periodic row overflow");
    let values = rows
        .checked_mul(columns.len())
        .expect("periodic table overflow");
    assert!(
        values <= pcs.limits.max_evaluation_values,
        "periodic resource limit exceeded"
    );
}
