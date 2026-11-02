// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private masked coefficients and caller-bounded natural-order evaluation tiles.
//! This is storage groundwork, not a PCS, memory bound or selected native profile.
mod coefficients;
mod evaluate;
mod masking;
mod strips;

pub use coefficients::MaskedTrace;
pub use strips::{QUOTIENT_STRIP_COUNT, QuotientStrips};
