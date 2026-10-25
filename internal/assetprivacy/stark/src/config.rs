// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_challenger::HashChallenger;
use rand::{
    SeedableRng,
    rngs::{StdRng, SysError, SysRng},
};

use crate::hash::DomainHash;
use crate::parameters::{RANDOM_CODEWORDS, fri};
use crate::types::{ChallengeMmcs, Challenger, Compress, Config, LeafHash, Pcs, ValueMmcs};

/// Seeds every commitment/masking stream from OS-backed entropy.
/// This does not establish the security of an application relation.
pub fn prover_config() -> Result<Config, SysError> {
    let leaves = StdRng::try_from_rng(&mut SysRng)?;
    let masks = StdRng::try_from_rng(&mut SysRng)?;
    Ok(configuration(leaves, masks))
}

// Verification never consumes masking entropy. Keep this inaccessible to provers.
pub(crate) fn verifier_config() -> Config {
    configuration(StdRng::from_seed([0; 32]), StdRng::from_seed([0; 32]))
}

fn value_mmcs(rng: StdRng) -> ValueMmcs {
    ValueMmcs::new(
        LeafHash::new(DomainHash::<1>),
        Compress::new(DomainHash::<2>),
        0,
        rng,
    )
}

fn configuration(leaves: StdRng, masks: StdRng) -> Config {
    let leaves = value_mmcs(leaves);
    let extensions = ChallengeMmcs::new(leaves.clone());
    let pcs = Pcs::new(
        Default::default(),
        leaves,
        fri(extensions),
        RANDOM_CODEWORDS,
        masks,
    );
    let transcript = HashChallenger::new(b"NAP-CANDIDATE-1".to_vec(), DomainHash::<3>);
    Config::new(pcs, Challenger::new(transcript))
}
