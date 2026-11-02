// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Equal-height, bit-reversed compact MMCS commitments, without a full LDE.
mod access;
mod layers;
mod leaves;
mod salts;
mod shape;

use super::MaskedTrace;
use crate::compact::{SALT_ELEMS, Value32};
use p3_dft::Radix2Dit;
use p3_field::coset::TwoAdicMultiplicativeCoset;
use p3_symmetric::MerkleCap;
use rand::CryptoRng;

pub type TreeDigest = [u64; 8];
pub type TreeCap = MerkleCap<Value32, TreeDigest>;

/// Private salts and public digest layers; no implicit copying or serialization.
/// This is not a PCS and does not enforce single-use opening budgets.
pub struct StreamedTree {
    rows: usize,
    widths: Vec<usize>,
    salts: Vec<Vec<[Value32; SALT_ELEMS]>>,
    layers: Vec<Vec<TreeDigest>>,
}

impl StreamedTree {
    /// Commit masked matrices in stable input order with fresh independent salts.
    /// max_tile_values limits one FFT buffer, not aggregate RSS or scratch.
    pub fn commit<R: CryptoRng>(
        inputs: &[&MaskedTrace<Value32>],
        domain: TwoAdicMultiplicativeCoset<Value32>,
        tile_columns: usize,
        max_tile_values: usize,
        dft: &Radix2Dit<Value32>,
        rng: &mut R,
    ) -> Option<Self> {
        let shape = shape::Shape::new(inputs, domain, tile_columns, max_tile_values)?;
        let salts = salts::draw(shape.rows, inputs.len(), rng)?;
        let leaves = leaves::hash(inputs, domain, tile_columns, max_tile_values, dft, &salts)?;
        let layers = layers::build(leaves)?;
        Some(Self {
            rows: shape.rows,
            widths: shape.widths,
            salts,
            layers,
        })
    }
}
