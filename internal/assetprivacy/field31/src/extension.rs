// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Value32;
use p3_field::extension::{
    Binomial, BinomiallyExtendable, ExtensionAlgebra, HasTwoAdicBinomialExtension, binomial_mul,
};
use p3_field::{PrimeCharacteristicRing, TwoAdicField};

impl ExtensionAlgebra<Self, 10, Binomial<Self>> for Value32 {
    fn ext_mul(a: &[Self; 10], b: &[Self; 10], result: &mut [Self; 10]) {
        result.fill(Self::ZERO);
        binomial_mul(a, b, result, Self::new(11));
    }
}

impl BinomiallyExtendable<10> for Value32 {
    fn binomial_algebra_id() -> Vec<u8> {
        b"algorand-fp31-power-basis-v1:p=2013265921:X^10-11".to_vec()
    }
    const W: Self = Self::new(11);
    const DTH_ROOT: Self = Self::new(1403701133);
    const EXT_GENERATOR: [Self; 10] = [
        Self::new(2),
        Self::new(1),
        Self::new(0),
        Self::new(0),
        Self::new(0),
        Self::new(0),
        Self::new(0),
        Self::new(0),
        Self::new(0),
        Self::new(0),
    ];
}

impl HasTwoAdicBinomialExtension<10> for Value32 {
    const EXT_TWO_ADICITY: usize = 28;
    fn ext_two_adic_generator(bits: usize) -> [Self; 10] {
        assert!(
            bits <= Self::EXT_TWO_ADICITY,
            "unsupported extension root order"
        );
        let mut value = [Self::ZERO; 10];
        if bits <= Self::TWO_ADICITY {
            value[0] = Self::two_adic_generator(bits);
        } else {
            value[5] = Self::new(17094607);
        }
        value
    }
}
