// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Cap, Config, Domain, V, pcs};
use p3_commit::Pcs;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;
use p3_uni_stark::StarkGenericConfig;

const ROWS: usize = 65536;
const WIDTH: usize = 3384;

pub(super) fn limits() -> pcs::Limits {
    pcs::Limits {
        tile_columns: 64,
        max_tile_values: (1 << 20) * 64,
        max_coefficient_values: 131072 * (WIDTH + 10),
        max_evaluation_values: 500_000_000,
        max_claim_values: 2 * (WIDTH + 10) * 10,
        max_query_values: 192 * (WIDTH + 10),
    }
}

pub(super) fn commit(config: &Config) -> (Cap, pcs::ProverData, Domain) {
    let domain = config.pcs().natural_domain_for_degree(2 * ROWS);
    println!("stage=masked_commit rows={ROWS} relation_width={WIDTH}");
    let (cap, data) = config
        .pcs()
        .commit([(domain, trace())])
        .expect("bounded commit");
    println!("stage=full_commit_ready ood_points=2 queries=192");
    (cap, data, domain)
}

fn trace() -> RowMajorMatrix<V> {
    let mut values = vec![V::ZERO; ROWS * WIDTH];
    for (row, cells) in values.as_chunks_mut::<WIDTH>().0.iter_mut().enumerate() {
        cells[0] = V::from_usize(row + 1);
        cells[WIDTH - 1] = V::from_usize(row + 7);
    }
    RowMajorMatrix::new(values, WIDTH)
}
