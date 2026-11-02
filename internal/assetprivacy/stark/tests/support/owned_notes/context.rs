// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Air, pipeline};
use algorand_asset_privacy_stark::compact::Config;
use p3_batch_stark::{BatchProof, ProverData};

pub fn reject(config: &Config, air: &Air, proof: &BatchProof<Config>, data: &ProverData<Config>) {
    let scopes = [
        ([4; 32], 7, [19; 64], [41; 64]),
        ([3; 32], 8, [19; 64], [41; 64]),
        ([3; 32], 7, [20; 64], [41; 64]),
        ([3; 32], 7, [19; 64], [42; 64]),
    ];
    for (network, asset, policy, group) in scopes {
        let other = Air::new(network, asset, policy, group, air.note_count(), air.rows())
            .expect("valid alternate verifier scope");
        assert!(!pipeline::accepts(config, &other, proof, data));
    }
}
