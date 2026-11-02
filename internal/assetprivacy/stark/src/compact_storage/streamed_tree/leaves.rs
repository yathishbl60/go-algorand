// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MaskedTrace, Radix2Dit, SALT_ELEMS, TreeDigest, TwoAdicMultiplicativeCoset, Value32};
use crate::compact::{compact_hash_state, compact_hash_words};
use p3_field::PrimeField32;
use p3_matrix::dense::RowMajorMatrix;
use sha3::{Digest, Sha3_512};

pub(super) fn hash(
    inputs: &[&MaskedTrace<Value32>],
    domain: TwoAdicMultiplicativeCoset<Value32>,
    tile_columns: usize,
    budget: usize,
    dft: &Radix2Dit<Value32>,
    salts: &[Vec<[Value32; SALT_ELEMS]>],
) -> Option<Vec<TreeDigest>> {
    let mut states = new_states(domain.size())?;
    for (input, salt) in inputs.iter().zip(salts) {
        absorb_matrix(&mut states, input, domain, tile_columns, budget, dft)?;
        absorb_salts(&mut states, salt)?;
    }
    let fields: usize = inputs.iter().map(|input| input.width() + SALT_ELEMS).sum();
    finish(states, fields % 2 == 1)
}

fn new_states(rows: usize) -> Option<Vec<Sha3_512>> {
    let mut states = Vec::new();
    states.try_reserve_exact(rows).ok()?;
    states.resize_with(rows, compact_hash_state::<1>);
    Some(states)
}

fn absorb_matrix(
    states: &mut [Sha3_512],
    input: &MaskedTrace<Value32>,
    domain: TwoAdicMultiplicativeCoset<Value32>,
    columns: usize,
    budget: usize,
    dft: &Radix2Dit<Value32>,
) -> Option<()> {
    for start in (0..input.width()).step_by(columns) {
        let end = start + columns.min(input.width() - start);
        let tile = input.evaluate_columns(start..end, domain, budget, dft)?;
        absorb_tile(states, &tile, domain.log_size())?;
    }
    Some(())
}

fn absorb_tile(states: &mut [Sha3_512], tile: &RowMajorMatrix<Value32>, log: usize) -> Option<()> {
    for (natural, row) in tile.values.chunks_exact(tile.width).enumerate() {
        let reversed = natural.reverse_bits() >> (usize::BITS as usize - log);
        let state = states.get_mut(reversed)?;
        for value in row {
            state.update(value.as_canonical_u32().to_le_bytes());
        }
    }
    Some(())
}

fn absorb_salts(states: &mut [Sha3_512], salts: &[[Value32; SALT_ELEMS]]) -> Option<()> {
    if states.len() != salts.len() {
        return None;
    }
    for (state, row) in states.iter_mut().zip(salts) {
        for value in row {
            state.update(value.as_canonical_u32().to_le_bytes());
        }
    }
    Some(())
}

fn finish(states: Vec<Sha3_512>, pad: bool) -> Option<Vec<TreeDigest>> {
    let mut digests = Vec::new();
    digests.try_reserve_exact(states.len()).ok()?;
    for mut state in states {
        // SerializingHasher's u64 stream pads ONLY the complete combined row.
        if pad {
            state.update([0; 4]);
        }
        digests.push(compact_hash_words(state.finalize().into()));
    }
    Some(digests)
}
