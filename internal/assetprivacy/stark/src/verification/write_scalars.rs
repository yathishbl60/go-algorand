// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Cap, Error, writer::Writer};
use crate::{Challenge, Value};
use p3_field::{BasedVectorSpace, PrimeCharacteristicRing, PrimeField64};

impl Writer {
    pub(super) fn value(&mut self, value: &Value) -> Result<(), Error> {
        self.push(&value.as_canonical_u64().to_le_bytes())
    }

    pub(super) fn challenge(&mut self, value: &Challenge) -> Result<(), Error> {
        for coefficient in value.as_basis_coefficients_slice() {
            self.value(coefficient)?;
        }
        Ok(())
    }

    pub(super) fn digest(&mut self, words: &[u64; 8]) -> Result<(), Error> {
        for word in words {
            self.push(&word.to_le_bytes())?;
        }
        Ok(())
    }

    pub(super) fn cap(&mut self, cap: &Cap) -> Result<(), Error> {
        if cap.num_roots() != 1 {
            return Err(Error::Shape);
        }
        self.digest(&cap.roots()[0])
    }

    pub(super) fn zero(&mut self, value: &Value) -> Result<(), Error> {
        if *value != Value::ZERO {
            return Err(Error::NonCanonical);
        }
        self.value(value)
    }

    pub(super) fn values(&mut self, values: &[Value]) -> Result<(), Error> {
        self.list(values, Self::value)
    }

    pub(super) fn challenges(&mut self, values: &[Challenge]) -> Result<(), Error> {
        self.list(values, Self::challenge)
    }
}
