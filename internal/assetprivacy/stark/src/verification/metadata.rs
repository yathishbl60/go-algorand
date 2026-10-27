// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Artifact, shape::Shape};
use sha3::{Digest, Sha3_512};

// Changing backend parameters, relation revision or codec requires a new identity.
const BACKEND: &[u8] =
    b"P3/1abcaa0795bc30ded29d54d0bc2cf495b6a20c2f/NullifierV2/Goldilocks-X5-3/SHA3-512/FRI-ZK/v1";

pub(super) fn identity(artifact: &Artifact, shape: Shape) -> [u8; 64] {
    let mut hash = Sha3_512::new();
    hash.update(b"Algorand/AssetPrivacy/Candidate1/VerifierArtifactV2/");
    hash.update(BACKEND);
    hash.update(artifact.reference);
    hash.update(artifact.profile);
    hash.update(artifact.registry);
    hash.update(artifact.relation);
    dimensions(&mut hash, artifact, shape);
    for word in artifact.key.commitment.roots()[0] {
        hash.update(word.to_le_bytes());
    }
    hash.finalize().into()
}

fn dimensions(hash: &mut Sha3_512, artifact: &Artifact, shape: Shape) {
    for value in [
        artifact.rows,
        shape.width,
        shape.preprocessed,
        shape.public,
        crate::NUM_QUERIES,
        crate::RANDOM_CODEWORDS,
        crate::MIN_TRACE_ROWS,
    ] {
        hash.update((value as u32).to_le_bytes());
    }
    // Blowup=8, final length=1, binary folds, cap height=0, salt width=5.
    // All four grinding budgets are zero; extension degree=5; wire version=1.
    hash.update([3, 0, 1, 0, 5, 0, 0, 0, 0, 5, 1]);
    hash.update([u8::from(shape.main_next), u8::from(shape.preprocessed_next)]);
}
