// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Domain, Error, V};
use crate::compact_storage::{CoefficientInputData, MaskedTrace};
use std::sync::{Arc, atomic::AtomicBool};

/// Clones share private coefficients and the exact same one-use opening lease.
#[derive(Clone)]
pub struct ProverData {
    pub(crate) inner: Arc<CommitmentData>,
}

pub(crate) struct CommitmentData {
    pub(crate) input: CoefficientInputData,
    pub(crate) sources: Vec<Arc<MaskedTrace<V>>>,
    pub(crate) widths: Vec<usize>,
    pub(crate) lde_domain: Domain,
    pub(crate) preprocessed: bool,
    pub(super) opened: AtomicBool,
}

impl ProverData {
    pub(crate) fn new(
        input: CoefficientInputData,
        sources: Vec<Arc<MaskedTrace<V>>>,
        lde_domain: Domain,
    ) -> Result<Self, Error> {
        let widths = relation_widths(&sources)?;
        Ok(Self {
            inner: Arc::new(CommitmentData {
                input,
                sources,
                widths,
                lde_domain,
                preprocessed: false,
                opened: AtomicBool::new(false),
            }),
        })
    }
}

fn relation_widths(sources: &[Arc<MaskedTrace<V>>]) -> Result<Vec<usize>, Error> {
    let mut widths = Vec::new();
    widths
        .try_reserve_exact(sources.len())
        .map_err(|_| Error::Allocation)?;
    widths.extend(sources.iter().map(|source| source.relation_width()));
    Ok(widths)
}
