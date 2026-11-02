// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Air;
use sha3::{Digest, Sha3_512};

pub fn air(count: usize, rows: usize) -> Air {
    Air::new([3; 32], 7, [19; 64], [41; 64], count, rows).expect("valid public scope")
}

pub fn inputs(count: usize, offset: u8) -> (Vec<[u8; 512]>, Vec<[u8; 32]>) {
    let secrets: Vec<_> = (0..count)
        .map(|i| core::array::from_fn(|j| offset.wrapping_add((i * 37 + j + 1) as u8)))
        .collect();
    let notes = secrets
        .iter()
        .enumerate()
        .map(|(i, key)| note(i, key))
        .collect();
    (notes, secrets)
}

fn note(index: usize, secret: &[u8; 32]) -> [u8; 512] {
    let mut note = [0; 512];
    note[0] = 1;
    note[1..9].copy_from_slice(&7u64.to_be_bytes());
    note[9..73].fill(19);
    note[73..137].copy_from_slice(&owner(secret));
    note[137..169].fill(61 + index as u8);
    let amount = if index == 0 { 0 } else { u64::MAX };
    note[169..177].copy_from_slice(&amount.to_be_bytes());
    note[177..209].fill(71 + index as u8);
    note[209..241].fill(81 + index as u8);
    note
}

fn owner(secret: &[u8; 32]) -> [u8; 64] {
    Sha3_512::new()
        .chain_update(b"Algorand/AssetPrivacy/Candidate1/SpendingKey/")
        .chain_update([3; 32])
        .chain_update(secret)
        .finalize()
        .into()
}
