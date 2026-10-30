// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{OwnerRegistrationAir, PUBLIC_VALUES};
use crate::{Value, sha3_air::digest_values};

pub struct PublicStatement {
    pub owner: [u8; 64],
    pub group: [u8; 64],
}

impl PublicStatement {
    pub(super) fn validate(&self) -> Option<()> {
        (self.owner != [0; 64] && self.group != [0; 64]).then_some(())
    }
}

impl OwnerRegistrationAir {
    pub fn public_values(&self, statement: &PublicStatement) -> Option<Vec<Value>> {
        statement.validate()?;
        let mut values = Vec::with_capacity(PUBLIC_VALUES);
        values.extend(digest_values(statement.owner));
        values.extend(digest_values(statement.group));
        Some(values)
    }
}
