// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Canonical ML-KEM coefficient arithmetic over q=3329, not a complete KEM.
//! All components add real bounded integer constraints over Goldilocks.

mod add;
mod canonical;
mod compression;
mod mul;
mod sub;

pub use add::{Add, eval_add};
pub use canonical::{Canonical, eval_canonical};
pub use compression::{Compression, eval_compression};
pub use mul::{Mul, eval_mul};
pub use sub::{Sub, eval_sub};

pub const Q: u16 = 3329;
