// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Optional scalar Fp31/D10 arithmetic; no native proof profile is selected.
//! The only authored unsafe syntax is the ten required QuotientMap declarations.
#![forbid(unsafe_op_in_unsafe_fn)]

mod assignments;
mod degree;
mod extension;
mod integers;
mod iterators;
mod operators;
mod prime;
mod ring;
mod serialization;

pub use degree::has_full_extension_degree;
use p3_baby_bear::BabyBear;
use p3_field::extension::BinomialExtensionField;

/// Scalar representation with canonical serialization and no SIMD packing.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Value32(pub(crate) BabyBear);

/// Power basis modulo X^10-11; public arithmetic metadata is independently checked.
pub type Challenge32 = BinomialExtensionField<Value32, 10>;

impl Value32 {
    /// Reduces any u32 modulo the prime 2013265921.
    pub const fn new(value: u32) -> Self {
        Self(BabyBear::new(value))
    }
}

const _: () = assert!(core::mem::size_of::<Value32>() == 4);
const _: () = assert!(core::mem::size_of::<Challenge32>() == 40);
