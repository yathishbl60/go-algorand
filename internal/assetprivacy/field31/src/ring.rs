// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Value32;
use num_bigint::BigUint;
use p3_baby_bear::BabyBear;
use p3_field::UniformSamplingField;
use p3_field::{Field, Packable, PrimeCharacteristicRing, PrimeField32, RawDataSerializable};

impl PrimeCharacteristicRing for Value32 {
    type PrimeSubfield = Self;
    const ZERO: Self = Self(BabyBear::ZERO);
    const ONE: Self = Self(BabyBear::ONE);
    const TWO: Self = Self(BabyBear::TWO);
    const NEG_ONE: Self = Self(BabyBear::NEG_ONE);
    fn from_prime_subfield(value: Self) -> Self {
        value
    }
    fn halve(&self) -> Self {
        Self(self.0.halve())
    }
}

impl Packable for Value32 {}

impl RawDataSerializable for Value32 {
    const NUM_BYTES: usize = 4;
    fn into_bytes(self) -> impl IntoIterator<Item = u8> {
        self.as_canonical_u32().to_le_bytes()
    }
}

impl Field for Value32 {
    type Packing = Self;
    const GENERATOR: Self = Self(BabyBear::GENERATOR);
    fn try_inverse(&self) -> Option<Self> {
        self.0.try_inverse().map(Self)
    }
    fn order() -> BigUint {
        BabyBear::order()
    }
}

impl UniformSamplingField for Value32 {
    const MAX_SINGLE_SAMPLE_BITS: usize = BabyBear::MAX_SINGLE_SAMPLE_BITS;
    const SAMPLING_BITS_M: [u64; 64] = BabyBear::SAMPLING_BITS_M;
}
