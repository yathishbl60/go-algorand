// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Value32;
use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

macro_rules! assignment_operator {
    ($trait:ident, $method:ident, $operator:tt) => {
        impl $trait for Value32 {
            fn $method(&mut self, rhs: Self) {
                self.0 $operator rhs.0;
            }
        }
    };
}

assignment_operator!(AddAssign, add_assign, +=);
assignment_operator!(SubAssign, sub_assign, -=);
assignment_operator!(MulAssign, mul_assign, *=);
assignment_operator!(DivAssign, div_assign, /=);
