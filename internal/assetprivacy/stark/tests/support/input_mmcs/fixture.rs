// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    CoefficientInputData, Compress, Dense, Domain, Fixture, Leaf, Mask, Stock, TreeCap, V,
};
use algorand_asset_privacy_stark::{
    compact::CompactHash,
    compact_storage::{InputMmcs, StreamedInputLimits},
};
use p3_dft::{Radix2Dit, TwoAdicSubgroupDft};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::{Matrix, bitrev::BitReversibleMatrix};
use rand::{SeedableRng, rngs::StdRng};
use std::sync::Arc;

pub fn new(widths: &[usize]) -> Fixture {
    let sources: Vec<_> = widths
        .iter()
        .enumerate()
        .map(|(index, &width)| masked(512, width, index))
        .collect();
    let domain = domain(10);
    let expected: Vec<_> = sources.iter().map(|source| dense(source, domain)).collect();
    let total_width: usize = sources.iter().map(|source| source.width()).sum();
    let input = InputMmcs::new().expect("OS entropy");
    let (cap, data) = input
        .commit_coefficients(
            sources,
            domain,
            limits(domain.size(), total_width * domain.size()),
        )
        .expect("typed coefficient commitment");
    assemble(input, (cap, data), expected)
}

fn assemble(
    input: InputMmcs,
    committed: (TreeCap, CoefficientInputData),
    expected: Vec<Dense>,
) -> Fixture {
    let (cap, data) = committed;
    let dimensions = expected.iter().map(Matrix::dimensions).collect();
    Fixture {
        input,
        oracle: oracle(),
        cap,
        data,
        dimensions,
        expected,
    }
}

pub fn domain(log: usize) -> Domain {
    Domain::new(V::from_u8(3), log).expect("evaluation coset")
}

pub fn limits(rows: usize, output: usize) -> StreamedInputLimits {
    StreamedInputLimits {
        tile_columns: 3,
        max_tile_values: rows * 3,
        max_output_values: output,
    }
}

pub fn masked(rows: usize, width: usize, index: usize) -> Arc<Mask> {
    let values = (0..rows * width)
        .map(|i| V::from_usize(index * 1024 + i * 7 + 13))
        .collect();
    let raw = Dense::new(values, width);
    let source = Domain::new(V::from_u8(5), rows.ilog2() as usize).expect("source coset");
    // Public fixture masks; InputMmcs always obtains its own OS-seeded salt state.
    let mut rng = StdRng::seed_from_u64(91 + index as u64);
    Arc::new(Mask::from_trace(raw, source, &Radix2Dit::default(), &mut rng).expect("fixture mask"))
}

pub fn oracle() -> Stock {
    // This seed is verifier-only; the oracle never commits private values here.
    Stock::new(
        Leaf::new(CompactHash::<1>),
        Compress::new(CompactHash::<2>),
        0,
        StdRng::seed_from_u64(61),
    )
}

fn dense(source: &Mask, domain: Domain) -> Dense {
    let mut coefficients = source.coefficients().clone();
    coefficients
        .values
        .resize(domain.size() * source.width(), V::ZERO);
    let natural = Radix2Dit::default().coset_dft_batch(coefficients, domain.shift());
    natural.bit_reverse_rows().to_row_major_matrix()
}
