// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{KeyMaterial, RegisteredKeyAir, RegisteredKeyInput, record::RecordAir};

pub(super) struct Prepared<'a> {
    pub input: RegisteredKeyInput<'a>,
    pub leaf: [u8; 64],
    pub coefficients: [u16; 768],
    pub rho: [u8; 32],
    pub owner: [u8; 64],
}

impl<'a> Prepared<'a> {
    pub fn new(
        air: &RegisteredKeyAir,
        input: RegisteredKeyInput<'a>,
        root: [u8; 64],
    ) -> Option<Self> {
        let leaf = leaf(air, input)?;
        if air.membership.root(leaf, input.path) != root {
            return None;
        }
        let (coefficients, rho, owner) = material(input.material);
        Some(Self {
            input,
            leaf,
            coefficients,
            rho,
            owner,
        })
    }
}

fn material(input: KeyMaterial<'_>) -> ([u16; 768], [u8; 32], [u8; 64]) {
    match input {
        KeyMaterial::View(key) => {
            let coefficients = core::array::from_fn(|i| coefficient(key, i));
            (
                coefficients,
                key[1152..].try_into().expect("validated key"),
                [0; 64],
            )
        }
        KeyMaterial::Spend(owner) => ([0; 768], [0; 32], owner),
    }
}

fn coefficient(key: &[u8], index: usize) -> u16 {
    let start = index / 2 * 3;
    let packed = u32::from_le_bytes([key[start], key[start + 1], key[start + 2], 0]);
    ((packed >> (12 * (index % 2))) & 4095) as u16
}

fn leaf(air: &RegisteredKeyAir, input: RegisteredKeyInput<'_>) -> Option<[u8; 64]> {
    match (&air.record, input.material) {
        (RecordAir::View(air), KeyMaterial::View(key)) => {
            air.commitment(key, input.account, input.epoch, input.created)
        }
        (RecordAir::Spend(air), KeyMaterial::Spend(owner)) => {
            air.commitment(owner, input.account, input.epoch, input.created)
        }
        _ => None,
    }
}
