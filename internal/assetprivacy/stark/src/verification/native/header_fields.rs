// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{
    DecodeLimits, Error,
    bundle::BundleLimits,
    selection::{Domain, Pin},
};
use super::{cursor::Cursor, header::Context};

pub(super) fn pin(cursor: &mut Cursor<'_>) -> Result<Pin, Error> {
    if cursor.array::<5>()? != *b"NAPV\x04" {
        return Err(Error::Shape);
    }
    Ok(Pin {
        profile: cursor.array()?,
        implementation: cursor.array()?,
        digest: cursor.array()?,
    })
}

pub(super) fn context(cursor: &mut Cursor<'_>) -> Result<Context, Error> {
    let domain = Domain {
        network: cursor.array()?,
        asset: u64::from_be_bytes(cursor.array()?),
        policy: cursor.array()?,
    };
    let group = cursor.array()?;
    if domain.network == [0; 32]
        || domain.asset == 0
        || domain.policy == [0; 64]
        || group == [0; 64]
    {
        return Err(Error::PublicValues);
    }
    Ok(Context { domain, group })
}

pub(super) fn limits(cursor: &mut Cursor<'_>) -> Result<(BundleLimits, DecodeLimits), Error> {
    let bundle = cursor.u32()? as usize;
    let proof = cursor.u32()? as usize;
    let allocation = cursor.u32()? as usize;
    Ok((
        BundleLimits::new(bundle, proof, allocation)?,
        DecodeLimits::new(proof, allocation)?,
    ))
}
