// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::OwnedNotesAir;
use p3_field::PrimeField64;
use sha3::{Digest, Sha3_512};

const PROGRAM_DOMAIN: &[u8] = b"Algorand/AssetPrivacy/CompactOwnedNotesProgram/v1/";

impl<F: PrimeField64> OwnedNotesAir<F> {
    pub(super) fn bound_public_values(&self, asset: u64, policy: [u8; 64]) -> Vec<F> {
        let digest: [u8; 64] = self.program_hash(asset, policy).finalize().into();
        digest
            .as_chunks::<2>()
            .0
            .iter()
            .chain(self.group.as_chunks::<2>().0.iter())
            .map(|bytes| F::from_u16(u16::from_le_bytes([bytes[0], bytes[1]])))
            .collect()
    }

    fn program_hash(&self, asset: u64, policy: [u8; 64]) -> Sha3_512 {
        let mut hash = Sha3_512::new();
        hash.update(PROGRAM_DOMAIN);
        hash.update(F::ORDER_U64.to_le_bytes());
        hash.update(self.network);
        hash.update(asset.to_le_bytes());
        hash.update(policy);
        hash.update(self.group);
        hash.update((self.count as u64).to_le_bytes());
        hash.update((self.rows as u64).to_le_bytes());
        for word in self.input_base {
            hash.update(word.to_le_bytes());
        }
        self.hash_program(&mut hash);
        hash
    }

    fn hash_program(&self, hash: &mut Sha3_512) {
        hash.update((self.periodic.len() as u64).to_le_bytes());
        for column in &self.periodic {
            hash.update((column.len() as u64).to_le_bytes());
            for value in column {
                hash.update(value.as_canonical_u64().to_le_bytes());
            }
        }
    }
}
