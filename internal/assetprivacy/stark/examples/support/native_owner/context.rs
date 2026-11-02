// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{PUBLIC_TEST_SECRET, Result, files};
use algorand_asset_privacy_stark::{
    owner_registration_air::PublicStatement, verification::selection::Domain,
};
use sha3::{Digest, Sha3_512};
use std::path::Path;

pub struct Context {
    pub domain: Domain,
    pub group: [u8; 64],
}

impl Context {
    pub fn read(path: &Path) -> Result<Self> {
        let bytes = files::read::<168>(path)?;
        let context = Self {
            domain: Domain {
                network: bytes[..32].try_into()?,
                asset: u64::from_le_bytes(bytes[32..40].try_into()?),
                policy: bytes[40..104].try_into()?,
            },
            group: bytes[104..].try_into()?,
        };
        context.validate()?;
        Ok(context)
    }

    pub fn statement(&self) -> PublicStatement {
        PublicStatement {
            owner: owner(self.domain.network),
            group: self.group,
        }
    }

    fn validate(&self) -> Result<()> {
        if self.domain.network == [0; 32]
            || self.domain.asset == 0
            || self.domain.policy == [0; 64]
            || self.group == [0; 64]
        {
            return Err("fixture context contains a zero domain/group".into());
        }
        Ok(())
    }
}

fn owner(network: [u8; 32]) -> [u8; 64] {
    Sha3_512::new()
        .chain_update(b"Algorand/AssetPrivacy/Candidate1/SpendingKey/")
        .chain_update(network)
        .chain_update(PUBLIC_TEST_SECRET)
        .finalize()
        .into()
}
