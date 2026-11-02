// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DOMAIN, OWNER, OwnedNotesAir, SELECTED, factory::xor_byte};
use p3_field::PrimeField64;
use sha3::{Digest, Sha3_512};

impl<F: PrimeField64> OwnedNotesAir<F> {
    pub(super) fn valid_witness(&self, notes: &[[u8; 512]], secrets: &[[u8; 32]]) -> bool {
        notes.len() == self.count
            && secrets.len() == self.count
            && notes.iter().zip(secrets).all(|(note, secret)| {
                self.note.valid_opening(note)
                    && *secret != [0; 32]
                    && self.owner_digest(secret).as_slice() == &note[73..137]
            })
    }

    fn owner_digest(&self, secret: &[u8; 32]) -> [u8; 64] {
        let mut hash = Sha3_512::new();
        hash.update(DOMAIN);
        hash.update(self.network);
        hash.update(secret);
        hash.finalize().into()
    }

    pub(super) fn owner_input(&self, secret: &[u8; 32]) -> [u64; 25] {
        let mut state = self.input_base;
        for (index, &byte) in secret.iter().enumerate() {
            xor_byte(&mut state, index + 5, byte);
        }
        state
    }

    pub(super) fn write_owner(&self, row: &mut [F], index: usize, notes: &[[u8; 512]]) {
        let Some(position) = self.note.position(index) else {
            return;
        };
        let bytes = &notes[position.note_index()][73..137];
        for (word, pair) in bytes.as_chunks::<2>().0.iter().enumerate() {
            row[OWNER + word] = F::from_u16(u16::from_le_bytes([pair[0], pair[1]]));
        }
        let offset = position.byte_offset();
        if (74..=136).contains(&offset) && offset % 2 == 0 {
            row[SELECTED] = row[OWNER + (offset - 74) / 2];
        }
    }
}
