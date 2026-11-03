// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Cap, Config, Data, Domain, E, Points, V, fixture, rounds};
use p3_commit::{OpeningRequest, Pcs};
use p3_uni_stark::StarkGenericConfig;

pub fn run() {
    let config = fixture::config();
    let a = fixture::domain(&config, 512);
    let b = fixture::domain(&config, 1024);
    let domains = [a, b];
    let (caps, data) = commits(&config, domains);
    let points = points(a);
    let mut challenger = fixture::challenger(&config, &caps, 71);
    let requests = data
        .iter()
        .zip(&points)
        .map(|(data, points)| OpeningRequest::from((data, points.clone())))
        .collect();
    let (values, proof) = config.pcs().open(requests, &mut challenger).unwrap();
    shape(&values, &proof.0);
    let rounds = rounds::build(&caps, &domains, &points, &values);
    rounds::mutations(&config, rounds, &proof);
}

fn commits(config: &Config, domains: [Domain; 2]) -> ([Cap; 2], [Data; 2]) {
    let (a, da) = config
        .pcs()
        .commit([1, 3].map(|w| (domains[0], fixture::matrix(512, w, w))))
        .unwrap();
    let (b, db) = fixture::commit(config, 1024, 2, 9);
    ([a, b], [da, db])
}

fn points(domain: Domain) -> Points {
    let z = fixture::point(3);
    let next = z * E::from(domain.subgroup_generator());
    vec![
        vec![vec![z, next, z], vec![next, z]],
        vec![vec![z, fixture::point(8)]],
    ]
}

fn shape(values: &super::Claims, hidden: &super::Claims) {
    assert_eq!(values[0][0][0], values[0][0][2]);
    assert_eq!(values[0][0][0].len(), 1);
    assert_eq!(values[0][1][0].len(), 3);
    assert_eq!(values[1][0][0].len(), 2);
    for round in hidden {
        for matrix in round {
            for point in matrix {
                assert_eq!(point.len(), 10);
                assert!(point.iter().any(|value| *value != E::from(V::ZERO)));
            }
        }
    }
}

use p3_field::PrimeCharacteristicRing;
