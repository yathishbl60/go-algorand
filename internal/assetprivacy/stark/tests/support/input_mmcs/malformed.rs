// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Mask, fixture};
use algorand_asset_privacy_stark::compact_storage::{InputError, InputMmcs, StreamedInputLimits};
use std::sync::Arc;

pub fn reject() {
    let input = InputMmcs::new().expect("OS entropy");
    let source = fixture::masked(512, 1, 0);
    let domain = fixture::domain(10);
    let valid = fixture::limits(domain.size(), source.width());
    reject_shapes(&input, &source);
    reject_geometry(&input, &source);
    reject_limits(&input, &source);
    reject_queries(&input, source, valid);
}

fn reject_geometry(input: &InputMmcs, source: &Arc<Mask>) {
    let limits = fixture::limits(2048, 100);
    let short = fixture::domain(9);
    assert!(matches!(
        input.commit_coefficients(vec![Arc::clone(source)], short, limits),
        Err(InputError::Shape)
    ));
    let taller = fixture::masked(1024, 1, 1);
    let sources = vec![Arc::clone(source), taller];
    assert!(matches!(
        input.commit_coefficients(sources, fixture::domain(11), limits),
        Err(InputError::Shape)
    ));
}

fn reject_shapes(input: &InputMmcs, source: &Arc<Mask>) {
    let domain = fixture::domain(10);
    let valid = fixture::limits(domain.size(), source.width());
    assert!(matches!(
        input.commit_coefficients(Vec::new(), domain, valid),
        Err(InputError::Shape)
    ));
    assert!(matches!(
        input.commit_coefficients(vec![Arc::clone(source); 17], domain, valid),
        Err(InputError::Shape)
    ));
}

fn reject_limits(input: &InputMmcs, source: &Arc<Mask>) {
    let valid = fixture::limits(1024, source.width());
    let mut bad = valid;
    bad.tile_columns = 0;
    assert!(rejected(input, source, bad));
    bad = valid;
    bad.max_tile_values -= 1;
    assert!(rejected(input, source, bad));
    bad = valid;
    bad.max_output_values -= 1;
    assert!(rejected(input, source, bad));
}

fn rejected(input: &InputMmcs, source: &Arc<Mask>, limits: StreamedInputLimits) -> bool {
    matches!(
        input.commit_coefficients(vec![Arc::clone(source)], fixture::domain(10), limits),
        Err(InputError::Limits)
    )
}

fn reject_queries(input: &InputMmcs, source: Arc<Mask>, valid: StreamedInputLimits) {
    let domain = fixture::domain(10);
    let (cap, data) = input
        .commit_coefficients(vec![Arc::clone(&source)], domain, valid)
        .expect("one-query budget");
    assert!(matches!(
        input.try_open_multi(&[0, 0], &data),
        Err(InputError::Limits)
    ));
    assert!(matches!(
        input.try_open_batch(domain.size(), &data),
        Err(InputError::Shape)
    ));
    let (fresh, _) = input
        .clone()
        .commit_coefficients(vec![source], domain, valid)
        .expect("fresh cloned salt stream");
    assert_ne!(cap.roots(), fresh.roots());
}
