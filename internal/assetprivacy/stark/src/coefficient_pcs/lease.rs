// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Error, ProverData};
use std::sync::{Arc, Mutex, atomic::Ordering};

// Held only around checks/stores; no entropy, claims, FFT or allocation while locked.
static CONSUMPTION: Mutex<()> = Mutex::new(());

pub(crate) fn consume(data: &[&ProverData]) -> Result<(), Error> {
    reject_duplicates(data)?;
    let _guard = CONSUMPTION.lock().map_err(|_| Error::RandomState)?;
    if data
        .iter()
        .any(|data| data.inner.opened.load(Ordering::Acquire))
    {
        return Err(Error::OpeningUsed);
    }
    for data in data {
        data.inner.opened.store(true, Ordering::Release);
    }
    Ok(())
}

fn reject_duplicates(data: &[&ProverData]) -> Result<(), Error> {
    for (index, current) in data.iter().enumerate() {
        if data[..index]
            .iter()
            .any(|previous| Arc::ptr_eq(&current.inner, &previous.inner))
        {
            return Err(Error::OpeningUsed);
        }
    }
    Ok(())
}
