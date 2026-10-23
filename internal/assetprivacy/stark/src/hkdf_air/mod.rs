// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Candidate envelope HKDF-SHA3-512: 64-byte salt, 32-byte IKM, 32-byte output.
//! Salt, shared secret and derived AES key remain private trace data.

mod binding;
mod trace;
mod value;

use crate::{Value, hmac_air::HmacAir};
use p3_air::{Air, AirBuilder, BaseAir};

const INFO: &[u8] = b"Algorand/AssetPrivacy/Candidate1/AES256GCM";

#[derive(Clone)]
pub struct HkdfAir {
    extract: HmacAir,
    expand: HmacAir,
}

impl HkdfAir {
    pub fn new() -> Option<Self> {
        Some(Self {
            extract: HmacAir::new(32)?,
            expand: HmacAir::new(INFO.len() + 1)?,
        })
    }
    pub(crate) fn width(&self) -> usize {
        self.extract.width() + self.expand.width()
    }
}

impl BaseAir<Value> for HkdfAir {
    fn width(&self) -> usize {
        self.width()
    }
    fn num_public_values(&self) -> usize {
        0
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for HkdfAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_prk(builder);
        self.eval_info(builder);
    }
}
