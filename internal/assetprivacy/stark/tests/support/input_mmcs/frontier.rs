// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Compress, Dense, Fixture, Leaf, Proof, V, Values};
use algorand_asset_privacy_stark::compact::CompactHash;
use p3_commit::Mmcs;
use p3_merkle_tree::MerkleTreeMmcs;

type Unsalted = MerkleTreeMmcs<V, u64, Leaf, Compress, 2, 8>;

pub fn compare(fixture: &Fixture, values: &Values, proof: &Proof) {
    // Every row was deliberately queried in this public, single-matrix fixture.
    // Rebuild a stock tree over those exact salted rows without needing its RNG seed.
    let oracle = oracle();
    let (cap, data) = oracle.commit(vec![salted(values, proof)]);
    assert_eq!(cap.roots(), fixture.cap.roots());
    let indices = [1023, 0, 1, 7, 1023, 7];
    let (_, expected) = oracle.open_multi_batch(&indices, &data);
    let (_, actual) = fixture
        .input
        .try_open_multi(&indices, &fixture.data)
        .expect("pruned subset");
    assert_eq!(actual.1.sibling_hashes, expected.sibling_hashes);
}

fn oracle() -> Unsalted {
    Unsalted::new(
        Leaf::new(CompactHash::<1>),
        Compress::new(CompactHash::<2>),
        0,
    )
}

fn salted(values: &Values, proof: &Proof) -> Dense {
    let width = values[0][0].len() + 10;
    let mut combined = Vec::with_capacity(values.len() * width);
    for (rows, salts) in values.iter().zip(&proof.0) {
        assert_eq!((rows.len(), salts.len()), (1, 1));
        assert_eq!(salts[0].len(), 10);
        combined.extend_from_slice(&rows[0]);
        combined.extend_from_slice(&salts[0]);
    }
    Dense::new(combined, width)
}
