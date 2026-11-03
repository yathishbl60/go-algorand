// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CoefficientPcs, Error, Request, capacity, geometry};

pub(super) fn validate(
    pcs: &CoefficientPcs,
    rounds: &[Request<'_>],
    preprocessing: Option<usize>,
) -> Result<usize, Error> {
    pcs.limits.validate()?;
    geometry::parameters(pcs)?;
    if preprocessing.is_some() {
        return Err(Error::Unsupported);
    }
    if !(1..=4).contains(&rounds.len()) {
        return Err(Error::Shape);
    }
    let mut counts = capacity::Counts::new();
    for round in rounds {
        geometry::round(pcs, round)?;
        capacity::count(pcs, round, &mut counts)?;
    }
    capacity::buffers(pcs, &counts)
}
