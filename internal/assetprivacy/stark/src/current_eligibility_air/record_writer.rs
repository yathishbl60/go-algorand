// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CurrentEligibilityAir, EligibilityInput};
use crate::Value;

pub(super) fn write(
    air: &CurrentEligibilityAir,
    input: EligibilityInput<'_>,
    values: &mut [Value],
    shape: (usize, usize, usize),
) -> Option<[u8; 64]> {
    air.record
        .write_into(input.account, input.revision, input.updated, values, shape)
}
