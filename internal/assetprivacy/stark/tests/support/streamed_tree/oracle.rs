// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Compress, Dense, Domain, Leaf, Oracle, SALT_SEED, Stock, V};
use algorand_asset_privacy_stark::{compact::CompactHash, compact_storage::MaskedTrace};
use p3_commit::Mmcs;
use p3_dft::Radix2Dit;
use p3_matrix::{Matrix, bitrev::BitReversibleMatrix};
use rand::{SeedableRng, rngs::StdRng};

pub fn commit(inputs: &[MaskedTrace<V>], domain: Domain) -> Oracle {
    let matrices: Vec<_> = inputs.iter().map(|input| lde(input, domain)).collect();
    let dimensions = matrices.iter().map(Matrix::dimensions).collect();
    let mmcs = Stock::new(
        Leaf::new(CompactHash::<1>),
        Compress::new(CompactHash::<2>),
        0,
        StdRng::seed_from_u64(SALT_SEED),
    );
    let (cap, data) = mmcs.commit(matrices);
    Oracle {
        mmcs,
        cap,
        data,
        dimensions,
    }
}

fn lde(input: &MaskedTrace<V>, domain: Domain) -> Dense {
    let dft = Radix2Dit::default();
    let natural = input
        .evaluate_columns(
            0..input.width(),
            domain,
            domain.size() * input.width(),
            &dft,
        )
        .expect("complete stock comparison LDE");
    // Stock PCS commits bit-reversed rows. Use its matrix permutation implementation.
    natural.bit_reverse_rows().to_row_major_matrix()
}
