// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Domain, Opening, Oracle, SALT_SEED, V, corruption, fixture, oracle};
use algorand_asset_privacy_stark::compact_storage::{MaskedTrace, StreamedTree};
use p3_commit::Mmcs;
use p3_dft::Radix2Dit;
use rand::{SeedableRng, rngs::StdRng};

pub fn compare(widths: &[usize], log: usize, shift: u8) {
    let inputs = fixture::traces(widths);
    let domain = fixture::domain(log, shift);
    let oracle = oracle::commit(&inputs, domain);
    let references: Vec<&MaskedTrace<V>> = inputs.iter().collect();
    for tile in [1, 3, 64] {
        let tree = streamed(&references, domain, tile);
        assert_eq!(tree.cap().expect("root").roots(), oracle.cap.roots());
        assert_eq!(tree.rows(), domain.size());
        assert_eq!(
            tree.widths(),
            inputs.iter().map(MaskedTrace::width).collect::<Vec<_>>()
        );
        compare_paths(&tree, &oracle);
    }
}

fn streamed(inputs: &[&MaskedTrace<V>], domain: Domain, tile: usize) -> StreamedTree {
    StreamedTree::commit(
        inputs,
        domain,
        tile,
        domain.size() * tile,
        &Radix2Dit::default(),
        &mut StdRng::seed_from_u64(SALT_SEED),
    )
    .expect("bounded streamed commitment")
}

fn compare_paths(tree: &StreamedTree, oracle: &Oracle) {
    for index in [0, 1, 7, tree.rows() / 2 + 3, tree.rows() - 1] {
        let stock = oracle.mmcs.open_batch(index, &oracle.data);
        let salts = tree.salts(index).expect("in-range salts");
        let path = tree.path(index).expect("in-range authentication path");
        assert_eq!(salts, stock.opening_proof.0);
        assert_eq!(path, stock.opening_proof.1);
        let opening = Opening::new(stock.opened_values, (salts, path));
        assert!(
            oracle
                .mmcs
                .verify_batch(&oracle.cap, &oracle.dimensions, index, (&opening).into())
                .is_ok()
        );
        corruption::reject(oracle, index, &opening);
    }
    assert!(tree.salts(tree.rows()).is_none());
    assert!(tree.path(tree.rows()).is_none());
}
