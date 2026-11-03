// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{coefficient_pcs::Error, fixture};
use p3_commit::Pcs;
use p3_uni_stark::StarkGenericConfig;
use std::sync::Barrier;

pub fn run() {
    let config = fixture::config();
    let (_, data) = fixture::commit(&config, 512, 1, 5);
    let barrier = Barrier::new(2);
    let results = race(&config, &data, &barrier);
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter(|result| matches!(result, Err(Error::OpeningUsed)))
            .count(),
        1
    );
}

fn race(config: &super::Config, data: &super::Data, barrier: &Barrier) -> Vec<Result<(), Error>> {
    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..2)
            .map(|_| {
                let data = data.clone();
                scope.spawn(move || open(config, &data, barrier))
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect::<Vec<_>>()
    })
}

fn open(
    config: &super::Config,
    data: &super::coefficient_pcs::ProverData,
    barrier: &Barrier,
) -> Result<(), Error> {
    let mut challenger = config.initialise_challenger();
    barrier.wait();
    config
        .pcs()
        .open(
            vec![(data, vec![vec![fixture::point(4)]]).into()],
            &mut challenger,
        )
        .map(|_| ())
}
