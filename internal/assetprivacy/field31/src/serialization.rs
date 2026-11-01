// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Value32;
use p3_field::{PrimeField32, integers::QuotientMap};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

impl Serialize for Value32 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let value = self.as_canonical_u32();
        if serializer.is_human_readable() {
            serializer.serialize_u32(value)
        } else {
            value.to_le_bytes().serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for Value32 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = if deserializer.is_human_readable() {
            u32::deserialize(deserializer)?
        } else {
            u32::from_le_bytes(<[u8; 4]>::deserialize(deserializer)?)
        };
        Self::from_canonical_checked(value)
            .ok_or_else(|| D::Error::custom("noncanonical Fp31 value"))
    }
}
