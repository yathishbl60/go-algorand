// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{coefficient_pcs::Error, fixture};
use p3_commit::UnivariateStarkPcs;
use p3_uni_stark::StarkGenericConfig;

pub fn run() {
    let config = fixture::config();
    let domain = fixture::domain(&config, 512);
    let result = config
        .pcs()
        .commit_preprocessing([(domain, fixture::matrix(512, 1, 1))]);
    assert!(matches!(result, Err(Error::Unsupported)));
    let (_, data) = fixture::commit(&config, 512, 1, 1);
    let mut challenger = config.initialise_challenger();
    let before = challenger.clone();
    let requests = vec![fixture::request(&data, 1)];
    let result = config
        .pcs()
        .open_with_preprocessing(requests, &mut challenger, Some(0));
    assert!(matches!(result, Err(Error::Unsupported)));
    fixture::unchanged(&before, &challenger);
    fixture::open(&config, &data, 1, &mut challenger);
}
