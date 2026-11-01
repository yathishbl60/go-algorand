// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Value32;
use core::fmt::{Display, Formatter, Result};
use core::ops::{Add, Div, Mul, Neg, Sub};

macro_rules! binary_operator {
    ($trait:ident, $method:ident, $operator:tt) => {
        impl $trait for Value32 {
            type Output = Self;
            fn $method(self, rhs: Self) -> Self {
                Self(self.0 $operator rhs.0)
            }
        }
    };
}

binary_operator!(Add, add, +);
binary_operator!(Sub, sub, -);
binary_operator!(Mul, mul, *);
binary_operator!(Div, div, /);

impl Neg for Value32 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Display for Value32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&self.0, f)
    }
}
