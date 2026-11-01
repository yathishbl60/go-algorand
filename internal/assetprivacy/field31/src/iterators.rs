// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Value32;
use core::iter::{Product, Sum};
use p3_field::PrimeCharacteristicRing;
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

impl Sum for Value32 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.map(|value| value.0).sum())
    }
}

impl Product for Value32 {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.map(|value| value.0).product())
    }
}

impl<'a> Sum<&'a Self> for Value32 {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl<'a> Product<&'a Self> for Value32 {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

impl Distribution<Value32> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Value32 {
        Value32(self.sample(rng))
    }
}

impl From<bool> for Value32 {
    fn from(value: bool) -> Self {
        Self::from_bool(value)
    }
}
