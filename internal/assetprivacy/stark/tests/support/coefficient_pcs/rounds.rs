// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Cap, Claims, Config, Domain, E, Proof, Round, compact, fixture};
use p3_commit::Pcs;
use p3_uni_stark::StarkGenericConfig;

pub fn build(
    caps: &[Cap],
    domains: &[Domain],
    points: &[Vec<Vec<E>>],
    values: &Claims,
) -> Vec<Round> {
    caps.iter()
        .enumerate()
        .map(|(i, cap)| Round {
            commitment: cap.clone(),
            matrices: points[i]
                .iter()
                .zip(&values[i])
                .map(|(points, values)| matrix(domains[i], points, values))
                .collect(),
        })
        .collect()
}

fn matrix(domain: Domain, points: &[E], values: &[Vec<E>]) -> p3_commit::MatrixOpening<E, Domain> {
    p3_commit::MatrixOpening {
        domain,
        points: points
            .iter()
            .copied()
            .zip(values.iter().cloned())
            .map(Into::into)
            .collect(),
    }
}

pub fn accepts(config: &Config, rounds: Vec<Round>, proof: &Proof, context: usize) -> bool {
    let caps: Vec<_> = rounds.iter().map(|r| r.commitment.clone()).collect();
    let mut challenger = fixture::challenger(config, &caps, context);
    compact::prover_config()
        .unwrap()
        .pcs()
        .verify(rounds, proof, &mut challenger)
        .is_ok()
}

pub fn mutations(config: &Config, rounds: Vec<Round>, proof: &Proof) {
    assert!(accepts(config, rounds.clone(), proof, 71));
    assert!(!accepts(config, rounds.clone(), proof, 72));
    let mut changed = rounds.clone();
    changed[0].matrices[0].points[0].values[0] += E::ONE;
    assert!(!accepts(config, changed, proof, 71));
    let mut changed = proof.clone();
    changed.0[0][0][0][0] += E::ONE;
    assert!(!accepts(config, rounds.clone(), &changed, 71));
    let mut changed = rounds;
    changed[0].matrices.swap(0, 1);
    assert!(!accepts(config, changed, proof, 71));
}

use p3_field::PrimeCharacteristicRing;
