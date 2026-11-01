// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Error;
use crate::Value;
use p3_field::PrimeField64;

/// Convert typed witness-builder public cells to the native uint16 vector format.
/// This bounds/converts public data only; it never transforms private trace columns.
pub fn public_values_u16(values: &[Value]) -> Result<Vec<u16>, Error> {
    if values.len() > 65536 {
        return Err(Error::PublicValues);
    }
    let mut result = Vec::new();
    result
        .try_reserve_exact(values.len())
        .map_err(|_| Error::AllocationFailure)?;
    for value in values {
        result.push(u16::try_from(value.as_canonical_u64()).map_err(|_| Error::PublicValues)?);
    }
    Ok(result)
}
