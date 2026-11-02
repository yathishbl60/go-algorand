// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    Config, Domain, E, Matrix, V,
    evaluation::{Quotient, evaluate},
    fixture::Fixture,
};
use algorand_asset_privacy_stark::{compact::prover_config, compact_storage::QuotientStrips};
use p3_field::PrimeCharacteristicRing;

pub fn compare() {
    let config = prover_config().expect("OS entropy");
    for log_n in [5, 7, 9] {
        for shift in [V::from_u8(3), V::from_u8(11)] {
            compare_case(&config, log_n, shift);
        }
    }
}

fn compare_case(config: &Config, log_n: usize, shift: V) {
    let rows = 1usize << log_n;
    let air = Fixture {
        periodic: vec![(0..rows).map(|i| V::from_usize(i % 5)).collect()],
    };
    let trace = Domain::new(V::ONE, log_n).expect("trace domain");
    let full = Domain::new(shift, log_n + 4).expect("quotient domain");
    let coefficients = Matrix::new((0..4 * rows).map(|i| V::from_usize(3 * i + 7)).collect(), 2);
    let dense = evaluate(&coefficients, full);
    let evaluator = Quotient {
        config,
        air: &air,
        trace,
    };
    let expected = evaluator.evaluate(full, &dense);
    assert_eq!(strips(&evaluator, full, &coefficients), expected);
}

fn strips(evaluator: &Quotient<'_>, full: Domain<V>, coefficients: &Matrix) -> Vec<E> {
    let geometry = QuotientStrips::new(evaluator.trace, full).expect("disjoint16N geometry");
    check_geometry(&geometry, evaluator.trace.size(), full.size());
    let mut merged = vec![E::ZERO; full.size()];
    for strip in 0..8 {
        let domain = geometry.strip(strip).expect("strip domain");
        let values = evaluate(coefficients, domain);
        for (row, value) in evaluator.evaluate(domain, &values).into_iter().enumerate() {
            merged[geometry.global_row(strip, row).expect("interleaved row")] = value;
        }
    }
    merged
}

fn check_geometry(geometry: &QuotientStrips<V>, trace_rows: usize, total_rows: usize) {
    assert_eq!(geometry.strip_rows(), 2 * trace_rows);
    assert_eq!(geometry.total_rows(), total_rows);
    assert!(geometry.strip(8).is_none());
    assert!(geometry.global_row(0, geometry.strip_rows()).is_none());
}
