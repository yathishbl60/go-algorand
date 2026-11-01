// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Value32;
use p3_baby_bear::BabyBear;
use p3_field::integers::QuotientMap;

macro_rules! integer_map {
    ($($integer:ty),+ $(,)?) => {
        $(impl QuotientMap<$integer> for Value32 {
            fn from_int(value: $integer) -> Self {
                Self(<BabyBear as QuotientMap<$integer>>::from_int(value))
            }
            fn from_canonical_checked(value: $integer) -> Option<Self> {
                <BabyBear as QuotientMap<$integer>>::from_canonical_checked(value).map(Self)
            }
            /// # Safety
            /// The trait's canonical-input precondition applies; reduction is safe even otherwise.
            unsafe fn from_canonical_unchecked(value: $integer) -> Self {
                Self(<BabyBear as QuotientMap<$integer>>::from_int(value))
            }
        })+
    };
}

// P3 supplies usize/isize through blanket implementations over these ten types.
integer_map!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
