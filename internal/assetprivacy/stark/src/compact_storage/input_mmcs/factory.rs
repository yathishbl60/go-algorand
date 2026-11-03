// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Arc, CompactHash, Compress, InputError, InputMmcs, LeafHash, Mutex, Stock};
use rand::{
    SeedableRng,
    rngs::{StdRng, SysError, SysRng},
};

impl InputMmcs {
    /// Independent OS-seeded dense/streamed salt sources; no predictable seeding API.
    pub fn new() -> Result<Self, SysError> {
        let dense = StdRng::try_from_rng(&mut SysRng)?;
        let streamed = StdRng::try_from_rng(&mut SysRng)?;
        let stock = Stock::new(
            LeafHash::new(CompactHash::<1>),
            Compress::new(CompactHash::<2>),
            0,
            dense,
        );
        Ok(Self {
            stock: Arc::new(stock),
            random: Arc::new(Mutex::new(streamed)),
        })
    }

    pub(super) fn fork_random(&self) -> Result<StdRng, InputError> {
        let mut state = self.random.lock().map_err(|_| InputError::RandomState)?;
        // Only the fresh local seed is drawn while locked; FFT/hash work runs unlocked.
        Ok(StdRng::from_rng(&mut *state))
    }
}
