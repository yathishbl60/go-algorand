// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{
    Error,
    bundle::{Expected, verify_bundle},
};
use super::{Failure, request::Request, target::Target};

/// The caller supplies authenticated native context/registry selection, never peer requests.
/// Success establishes the exact proof set only, not transaction consent or execution.
pub fn verify_request(request: &[u8], bundle: &[u8]) -> Result<(), Failure> {
    let request = Request::parse(request).map_err(Failure::Request)?;
    let targets = targets(&request).map_err(Failure::Request)?;
    let expected = expected(&targets, &request).map_err(Failure::Request)?;
    super::super::bundle::validate_expected(&expected).map_err(Failure::Request)?;
    verify_bundle(
        bundle,
        &expected,
        request.header.bundle_limits,
        request.header.proof_limits,
    )
    .map_err(Failure::Proof)
}

fn targets(request: &Request) -> Result<Vec<Target>, Error> {
    let mut targets = Vec::new();
    targets
        .try_reserve_exact(request.slots.len())
        .map_err(|_| Error::AllocationFailure)?;
    for slot in &request.slots {
        targets.push(Target::build(
            &request.header.context,
            &request.header.registry,
            slot,
        )?);
    }
    Ok(targets)
}

fn expected<'a>(targets: &'a [Target], request: &'a Request) -> Result<Vec<Expected<'a>>, Error> {
    let mut slots = Vec::new();
    slots
        .try_reserve_exact(targets.len())
        .map_err(|_| Error::AllocationFailure)?;
    for (target, slot) in targets.iter().zip(&request.slots) {
        slots.push(target.expected(slot)?);
    }
    Ok(slots)
}
