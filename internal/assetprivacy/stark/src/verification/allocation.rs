// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Deterministic decoder charges under a fixed 64-bit storage model.
//! These charges are consensus-accounting units, not exact actual heap usage.
//! Child element storage is charged separately from each 24-byte Vec header.
//! Quota rejection is deterministic; allocator failure remains a local failure.

use super::InputOpening;
use crate::{Challenge, Value, types::ChallengeMmcs};
use p3_fri::CommitPhaseMultiStep;
use p3_symmetric::MerkleCap;

// This private module prevents external implementations or caller-selected charges.
pub(super) trait AllocationCharge {
    const BYTES: usize;
}

impl AllocationCharge for Value {
    const BYTES: usize = 8;
}

impl AllocationCharge for Challenge {
    const BYTES: usize = 40;
}

impl AllocationCharge for [u64; 8] {
    const BYTES: usize = 64;
}

impl<T> AllocationCharge for Vec<T> {
    const BYTES: usize = 24;
}

// The pinned PCS Cap alias normalizes to this concrete type.
impl AllocationCharge for MerkleCap<Value, [u64; 8]> {
    const BYTES: usize = 24;
}

impl AllocationCharge for InputOpening {
    const BYTES: usize = 72;
}

impl AllocationCharge for CommitPhaseMultiStep<Challenge, ChallengeMmcs> {
    const BYTES: usize = 72;
}
