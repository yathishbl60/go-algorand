// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Value;
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

pub(crate) const MESSAGE_BYTES: usize = 245;
pub(crate) const COMMITMENT_BYTE: usize = 181;

pub(crate) fn prefix(network: [u8; 32], asset: u64, policy: [u8; 64]) -> [u8; 149] {
    let mut prefix = [0; 149];
    prefix[..45].copy_from_slice(b"Algorand/AssetPrivacy/Candidate1/NullifierV2/");
    prefix[45..77].copy_from_slice(&network);
    prefix[77..85].copy_from_slice(&asset.to_be_bytes());
    prefix[85..].copy_from_slice(&policy);
    prefix
}

pub(crate) fn message(
    prefix: [u8; 149],
    opening: &[u8; 512],
    commitment: [u8; 64],
) -> [u8; MESSAGE_BYTES] {
    let mut message = [0; MESSAGE_BYTES];
    message[..149].copy_from_slice(&prefix);
    message[149..181].copy_from_slice(&opening[177..209]);
    message[COMMITMENT_BYTE..].copy_from_slice(&commitment);
    message
}

pub(crate) fn bind_commitment<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    gate: AB::Expr,
    limbs: &[AB::Var],
    bits: &[AB::Var],
) {
    assert_eq!(limbs.len(), 32, "private note commitment limbs");
    assert_eq!(bits.len(), 512, "private note commitment bits");
    for (limb, &value) in limbs.iter().enumerate() {
        let expected = (0..16).rev().fold(AB::Expr::ZERO, |acc, bit| {
            acc.double() + bits[limb * 16 + bit]
        });
        builder.when(gate.clone()).assert_eq(value, expected);
    }
}
