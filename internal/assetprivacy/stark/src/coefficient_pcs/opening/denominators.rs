// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Domain, E, Error, zeroes};
use p3_field::{PrimeCharacteristicRing, batch_multiplicative_inverse};

/// One opening point at a time; no retained global denominator cache.
pub(super) fn inverse(domain: Domain, point: E) -> Result<Vec<E>, Error> {
    let mut differences = zeroes(domain.size())?;
    for (index, x) in domain.iter().enumerate() {
        let reversed = index.reverse_bits() >> (usize::BITS as usize - domain.log_size());
        differences[reversed] = point - x;
    }
    if differences.contains(&E::ZERO) {
        return Err(Error::Shape);
    }
    Ok(batch_multiplicative_inverse(&differences))
}
