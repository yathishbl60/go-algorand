// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CompactKeccakAir, DOMAIN, NoteBytesAir, OwnedNotesAir};
use crate::serial_keccak::{chi_iota, rho_pi, theta};
use p3_field::PrimeField64;

pub(super) type Scope = ([u8; 32], u64, [u8; 64], [u8; 64], usize, usize);

impl<F: PrimeField64> OwnedNotesAir<F> {
    pub(super) fn construct(scope: Scope) -> Option<Self> {
        let (network, asset, policy, group, count, rows) = scope;
        let note = NoteBytesAir::new(asset, policy, count)?.padded_to(rows)?;
        let hash = CompactKeccakAir::new(count)?.padded_to(rows)?;
        let mut air = Self {
            note,
            hash,
            network,
            group,
            rows,
            count,
            input_base: input_base(network)?,
            periodic: Vec::new(),
            public_values: Vec::new(),
        };
        air.periodic = air.build_program()?;
        air.public_values = air.bound_public_values(asset, policy);
        Some(air)
    }
}

fn input_base(network: [u8; 32]) -> Option<[u64; 25]> {
    let mut prefix = [0u8; 77];
    prefix[..45].copy_from_slice(DOMAIN);
    prefix[45..].copy_from_slice(&network);
    let mut state = [0; 25];
    for (index, &byte) in prefix[..72].iter().enumerate() {
        xor_byte(&mut state, index, byte);
    }
    for round in 0..24 {
        state = chi_iota(rho_pi(theta(state)), round)?;
    }
    for (index, &byte) in prefix[72..].iter().enumerate() {
        xor_byte(&mut state, index, byte);
    }
    xor_byte(&mut state, 37, 0x06);
    xor_byte(&mut state, 71, 0x80);
    Some(state)
}

pub(super) fn xor_byte(state: &mut [u64; 25], index: usize, byte: u8) {
    state[index / 8] ^= u64::from(byte) << (8 * (index % 8));
}
