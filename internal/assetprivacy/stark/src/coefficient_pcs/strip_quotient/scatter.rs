// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{E, Error, V};
use crate::compact_storage::QuotientStrips;
use p3_field::PrimeCharacteristicRing;

pub(super) fn zeroes(rows: usize) -> Result<Vec<E>, Error> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(rows)
        .map_err(|_| Error::Allocation)?;
    values.resize(rows, E::ZERO);
    Ok(values)
}

pub(super) fn strip(
    strips: &QuotientStrips<V>,
    index: usize,
    values: &[E],
    output: &mut [E],
) -> Result<(), Error> {
    if values.len() != strips.strip_rows() || output.len() != strips.total_rows() {
        return Err(Error::Shape);
    }
    for (row, &value) in values.iter().enumerate() {
        let target = strips.global_row(index, row).ok_or(Error::Shape)?;
        output[target] = value;
    }
    Ok(())
}
