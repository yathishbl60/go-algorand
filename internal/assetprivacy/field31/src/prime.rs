// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Value32;
use num_bigint::BigUint;
use p3_baby_bear::BabyBear;
use p3_field::{PrimeField, PrimeField32, PrimeField64, TwoAdicField};

impl PrimeField for Value32 {
    fn as_canonical_biguint(&self) -> BigUint {
        self.0.as_canonical_biguint()
    }
}

impl PrimeField64 for Value32 {
    const ORDER_U64: u64 = BabyBear::ORDER_U64;
    fn as_canonical_u64(&self) -> u64 {
        self.0.as_canonical_u64()
    }
}

impl PrimeField32 for Value32 {
    const ORDER_U32: u32 = BabyBear::ORDER_U32;
    fn as_canonical_u32(&self) -> u32 {
        self.0.as_canonical_u32()
    }
}

impl TwoAdicField for Value32 {
    const TWO_ADICITY: usize = BabyBear::TWO_ADICITY;
    fn two_adic_generator(bits: usize) -> Self {
        assert!(
            bits <= Self::TWO_ADICITY,
            "unsupported base-field root order"
        );
        Self(BabyBear::two_adic_generator(bits))
    }
}
