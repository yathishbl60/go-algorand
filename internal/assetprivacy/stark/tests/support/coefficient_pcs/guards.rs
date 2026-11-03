// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Challenger, Config, Data, coefficient_pcs::Error, fixture};
use fixture::request;
use p3_commit::Pcs;
use p3_uni_stark::StarkGenericConfig;

pub fn run() {
    hiding_and_replay();
    duplicate_preserves_fresh();
    super::budgets::run();
    super::preprocessing::run();
}

fn hiding_and_replay() {
    let config = fixture::config();
    let (cap, data) = fixture::commit(&config, 512, 1, 1);
    let clone = data.clone();
    let mut challenger = fixture::challenger(&config, &[cap], 3);
    let before = challenger.clone();
    let result = config.pcs().open(vec![request(&data, 7)], &mut challenger);
    assert!(matches!(result, Err(Error::HidingBudget)));
    fixture::unchanged(&before, &challenger);
    fixture::open(&config, &clone, 6, &mut challenger);
    replay_rejected(&config, &data, &mut challenger);
    let (_, fresh) = fixture::commit(&config, 512, 1, 1);
    fixture::open(&config, &fresh, 1, &mut challenger);
}

fn replay_rejected(config: &Config, data: &Data, challenger: &mut Challenger) {
    let before = challenger.clone();
    let result = config.pcs().open(vec![request(data, 1)], challenger);
    assert!(matches!(result, Err(Error::OpeningUsed)));
    fixture::unchanged(&before, challenger);
}

fn duplicate_preserves_fresh() {
    let config = fixture::config();
    let (_, first) = fixture::commit(&config, 512, 1, 1);
    let (_, fresh) = fixture::commit(&config, 512, 1, 2);
    let clone = first.clone();
    let mut challenger = config.initialise_challenger();
    let before = challenger.clone();
    let requests = vec![request(&fresh, 1), request(&first, 1), request(&clone, 1)];
    assert!(config.pcs().open(requests, &mut challenger).is_err());
    fixture::unchanged(&before, &challenger);
    let requests = vec![request(&fresh, 1), request(&first, 1)];
    config.pcs().open(requests, &mut challenger).unwrap();
}
