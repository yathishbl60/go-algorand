// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Test-only canonical corruption without exposing a production proof decoder.
use super::{Owner, Result};
use algorand_asset_privacy_stark::{
    Value,
    owner_registration_air::WIDTH,
    verification::{DecodeLimits, Error, selection::Registry},
};
use p3_field::{PrimeCharacteristicRing, PrimeField64};
use std::ops::Range;

const MAGIC: &[u8; 8] = b"NAPSTK\x01\0";
const DIGEST_BYTES: usize = 8 * core::mem::size_of::<u64>();

pub fn proof(
    prepared: &Owner,
    registry: &Registry,
    valid: &[u8],
    values: &[u16],
) -> Result<Vec<u8>> {
    let mut changed = valid.to_vec();
    mutate(&mut changed)?;
    let verifier = registry.bind(prepared.clone())?;
    let limits = DecodeLimits::new(4_900_000, 64 << 20)?;
    if verifier.verify(&changed, values, limits) != Err(Error::InvalidProof) {
        return Err("corrupted opening did not reach cryptographic proof rejection".into());
    }
    Ok(changed)
}

fn mutate(proof: &mut [u8]) -> Result<()> {
    let range = first_opening(proof)?;
    let word = u64::from_le_bytes(proof[range.clone()].try_into()?);
    if word >= Value::ORDER_U64 {
        return Err("original proof coordinate is not canonical".into());
    }
    let changed = Value::from_u64(word) + Value::ONE;
    proof[range].copy_from_slice(&changed.as_canonical_u64().to_le_bytes());
    Ok(())
}

fn first_opening(proof: &[u8]) -> Result<Range<usize>> {
    if proof.get(..MAGIC.len()) != Some(MAGIC.as_slice()) {
        return Err("unexpected canonical proof marker".into());
    }
    // Canonical Writer::cap emits one fixed digest, with no length prefix.
    let header = MAGIC
        .len()
        .checked_add(DIGEST_BYTES + 1)
        .ok_or("header overflow")?;
    let count_at = header.checked_add(3 * DIGEST_BYTES).ok_or("cap overflow")?;
    let start = count_at.checked_add(4).ok_or("row prefix overflow")?;
    check_width(proof, count_at..start)?;
    let end = start
        .checked_add(core::mem::size_of::<u64>())
        .ok_or("coordinate overflow")?;
    proof
        .get(start..end)
        .ok_or("missing first opening coordinate")?;
    Ok(start..end)
}

fn check_width(proof: &[u8], prefix: Range<usize>) -> Result<()> {
    let encoded = proof.get(prefix).ok_or("missing row count")?;
    let count = u32::from_le_bytes(encoded.try_into()?);
    if usize::try_from(count)? != WIDTH {
        return Err("unexpected Owner opening row width".into());
    }
    Ok(())
}
