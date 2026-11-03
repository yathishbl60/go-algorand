// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Argument, Claims, Domain, E, Fixture, V};
use p3_commit::{CommitmentOpening, MatrixOpening, PointOpening};
use p3_field::PrimeCharacteristicRing;

pub fn expected(f: &Fixture, argument: &Argument, point: E) -> Claims {
    vec![CommitmentOpening {
        commitment: f.cap.clone(),
        matrices: vec![matrix(argument, point)],
    }]
}

fn matrix(argument: &Argument, point: E) -> MatrixOpening<E, Domain> {
    MatrixOpening {
        domain: Domain::new(V::ONE, 10).unwrap(),
        points: vec![PointOpening {
            point,
            values: argument.values.clone(),
        }],
    }
}
