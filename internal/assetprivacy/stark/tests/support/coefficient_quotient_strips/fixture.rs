// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Config, coefficient_pcs};

pub fn config(rows: usize, differential: bool) -> Config {
    if rows == 65536 {
        return full_height();
    }
    with_budget(if differential {
        100_000_000
    } else {
        rows / 512 * 3_000_000
    })
}

fn full_height() -> Config {
    coefficient_pcs::prover_config(coefficient_pcs::Limits {
        tile_columns: 64,
        max_tile_values: (65536 * 16) * 64,
        max_coefficient_values: 65536 * 2 * (1988 + 10),
        max_evaluation_values: 400_000_000,
        max_claim_values: 2_000_000,
        max_query_values: 5_000_000,
    })
    .unwrap()
}

pub fn with_budget(max_evaluation_values: usize) -> Config {
    let limits = coefficient_pcs::Limits {
        tile_columns: 16,
        max_tile_values: 4_194_304,
        max_coefficient_values: 20_000_000,
        max_evaluation_values,
        max_claim_values: 1_000_000,
        max_query_values: 5_000_000,
    };
    coefficient_pcs::prover_config(limits).unwrap()
}
