// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MESSAGE, OwnerRegistrationAir, PREFIX, PublicStatement, ROWS, WIDTH, WORK};
use crate::{Value, sha3_air::digest_values};
use p3_field::{Field, PrimeCharacteristicRing};
use p3_matrix::dense::RowMajorMatrix;
use sha3::{Digest, Sha3_512};

pub struct GroupWitness {
    pub trace: RowMajorMatrix<Value>,
    pub public_values: Vec<Value>,
}

impl OwnerRegistrationAir {
    pub fn trace(&self, secret: [u8; 32], statement: &PublicStatement) -> Option<GroupWitness> {
        let public_values = self.public_values(statement)?;
        let message = self.message(secret, statement.owner)?;
        let mut values = Value::zero_vec(ROWS * WIDTH);
        self.hash.write_into(&message, &mut values, WIDTH, WORK)?;
        write_globals(&mut values, secret, statement.group);
        Some(GroupWitness {
            trace: RowMajorMatrix::new(values, WIDTH),
            public_values,
        })
    }
    fn message(&self, secret: [u8; 32], owner: [u8; 64]) -> Option<[u8; MESSAGE]> {
        if secret == [0; 32] {
            return None;
        }
        let mut message = [0; MESSAGE];
        message[..PREFIX].copy_from_slice(&self.prefix);
        message[PREFIX..].copy_from_slice(&secret);
        let digest: [u8; 64] = Sha3_512::digest(message).into();
        (digest == owner).then_some(message)
    }
}

fn write_globals(values: &mut [Value], secret: [u8; 32], group: [u8; 64]) {
    let count: u32 = secret.iter().map(|byte| byte.count_ones()).sum();
    let inverse = Value::from_u32(count).inverse();
    let group = digest_values(group);
    for row in values.as_chunks_mut::<WIDTH>().0 {
        row[0] = inverse;
        row[1..WORK].copy_from_slice(&group);
    }
}
