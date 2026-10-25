// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Cap, Error, reader::Reader};
use crate::{Challenge, Value};
use p3_field::{PrimeCharacteristicRing, PrimeField64};

impl Reader<'_> {
    pub(super) fn value(&mut self) -> Result<Value, Error> {
        let word = self.word()?;
        if word >= Value::ORDER_U64 {
            return Err(Error::NonCanonical);
        }
        Ok(Value::from_u64(word))
    }

    pub(super) fn challenge(&mut self) -> Result<Challenge, Error> {
        let mut coefficients = [Value::ZERO; 5];
        for coefficient in &mut coefficients {
            *coefficient = self.value()?;
        }
        Ok(Challenge::new(coefficients))
    }

    pub(super) fn digest(&mut self) -> Result<[u64; 8], Error> {
        let mut words = [0; 8];
        for word in &mut words {
            *word = self.word()?;
        }
        Ok(words)
    }

    pub(super) fn cap(&mut self) -> Result<Cap, Error> {
        let digest = self.digest()?;
        let mut roots = self.allocate(1)?;
        roots.push(digest);
        Ok(Cap::new(roots))
    }

    pub(super) fn zero(&mut self) -> Result<Value, Error> {
        let value = self.value()?;
        if value != Value::ZERO {
            return Err(Error::NonCanonical);
        }
        Ok(value)
    }

    pub(super) fn values(&mut self) -> Result<Vec<Value>, Error> {
        self.list(8, Self::value)
    }

    pub(super) fn challenges(&mut self) -> Result<Vec<Challenge>, Error> {
        self.list(40, Self::challenge)
    }
}
