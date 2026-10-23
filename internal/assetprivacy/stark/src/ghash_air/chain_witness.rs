// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Gf128, Ghash, block::bits, field::multiply};
use crate::Value;

type Products<const N: usize> = Box<[Gf128<Value>; N]>;

impl<const N: usize> Ghash<Value, N> {
    /// Only whole blocks, with a positive verifier-fixed block count, are accepted.
    pub fn witness(hash_subkey: [u8; 16], blocks: [[u8; 16]; N]) -> Option<Self> {
        if N == 0 {
            return None;
        }
        let hash = u128::from_be_bytes(hash_subkey);
        let (products, accumulator) = products(hash, &blocks)?;
        Some(Self {
            hash_subkey: bits(hash),
            blocks: blocks.map(|block| bits(u128::from_be_bytes(block))),
            products,
            output: bits(accumulator),
        })
    }
}

fn products<const N: usize>(hash: u128, blocks: &[[u8; 16]; N]) -> Option<(Products<N>, u128)> {
    let mut accumulator = 0;
    let products: Vec<_> = blocks
        .iter()
        .map(|block| {
            let input = accumulator ^ u128::from_be_bytes(*block);
            let product = Gf128::from_integers(input, hash);
            accumulator = multiply(input, hash);
            product
        })
        .collect();
    Some((products.into_boxed_slice().try_into().ok()?, accumulator))
}
