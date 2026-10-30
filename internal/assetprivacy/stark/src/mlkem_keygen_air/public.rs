// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemKeygenAir, PUBLIC_VALUES};
use crate::{Value, sha3_air::digest_values};
use p3_field::PrimeCharacteristicRing;

pub struct PublicStatement {
    pub group: [u8; 64],
}

impl MlKemKeygenAir {
    pub fn public_values(&self, statement: &PublicStatement) -> Option<Vec<Value>> {
        if statement.group == [0; 64] {
            return None;
        }
        let mut values = Vec::with_capacity(PUBLIC_VALUES);
        values.extend(self.key.encoded.iter().map(|&byte| Value::from_u8(byte)));
        values.extend(digest_values(statement.group));
        Some(values)
    }
}
