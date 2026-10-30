// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{
    DecodeLimits, Error,
    bundle::BundleLimits,
    selection::{Domain, Registry},
};
use super::{HEADER, MAX_REQUEST_BYTES, cursor::Cursor};

pub(super) struct Context {
    pub domain: Domain,
    pub group: [u8; 64],
}
pub(super) struct Header {
    pub context: Context,
    pub registry: Registry,
    pub bundle_limits: BundleLimits,
    pub proof_limits: DecodeLimits,
    pub count: usize,
}

impl Header {
    pub(super) fn parse(bytes: &[u8]) -> Result<Self, Error> {
        if !(HEADER..=MAX_REQUEST_BYTES).contains(&bytes.len()) {
            return Err(Error::WireLimit);
        }
        let mut cursor = Cursor::new(&bytes[..HEADER]);
        let header = Self::read(&mut cursor)?;
        cursor.finish()?;
        Ok(header)
    }

    fn read(cursor: &mut Cursor<'_>) -> Result<Self, Error> {
        let pin = super::header_fields::pin(cursor)?;
        let context = super::header_fields::context(cursor)?;
        let (bundle_limits, proof_limits) = super::header_fields::limits(cursor)?;
        let registry = Registry::load(cursor.take(1118)?, pin)?;
        let count = usize::from(cursor.byte()?);
        if !(1..=7).contains(&count) {
            return Err(Error::Shape);
        }
        Ok(Self {
            context,
            registry,
            count,
            bundle_limits,
            proof_limits,
        })
    }
}
