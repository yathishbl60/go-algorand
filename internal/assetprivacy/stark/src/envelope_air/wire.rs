// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{EnvelopeAir, PUBLIC_VALUES};
use crate::{Value, gcm_vertical::GcmVerticalAir, mlkem_encapsulation_air::MlKemEncapsulationAir};
use p3_field::PrimeCharacteristicRing;

impl EnvelopeAir {
    pub fn public_values(&self, envelope: &[u8; 1629]) -> Option<[Value; PUBLIC_VALUES]> {
        if envelope[0] != 1 {
            return None;
        }
        let kem = MlKemEncapsulationAir.public_values(envelope[1..1089].try_into().ok()?);
        let gcm = GcmVerticalAir.public_values(
            envelope[1089..1101].try_into().ok()?,
            envelope[1101..1613].try_into().ok()?,
            envelope[1613..].try_into().ok()?,
        );
        Some(self.join_public(kem, gcm))
    }

    fn join_public(&self, kem: [Value; 544], gcm: [Value; 276]) -> [Value; PUBLIC_VALUES] {
        core::array::from_fn(|index| match index {
            0..248 => Value::from_u8(self.context.bytes()[index]),
            248..792 => kem[index - 248],
            _ => gcm[index - 792],
        })
    }
}

pub(super) fn encode(kem: [u8; 1088], nonce: [u8; 12], ct: [u8; 512], tag: [u8; 16]) -> [u8; 1629] {
    let mut envelope = [0; 1629];
    envelope[0] = 1;
    envelope[1..1089].copy_from_slice(&kem);
    envelope[1089..1101].copy_from_slice(&nonce);
    envelope[1101..1613].copy_from_slice(&ct);
    envelope[1613..].copy_from_slice(&tag);
    envelope
}
