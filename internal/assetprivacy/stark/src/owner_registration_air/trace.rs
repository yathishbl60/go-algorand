// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MESSAGE, OwnerRegistrationAir, PREFIX, PublicStatement, WIDTH, trace_writer};
use crate::Value;
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
        let values = trace_writer::build(&message, secret, statement.group)?;
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
