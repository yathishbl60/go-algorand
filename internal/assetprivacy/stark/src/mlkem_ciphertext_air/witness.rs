// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{COEFFICIENTS, LOCAL_BITS, REMAINDER, ROLLOVER, SCRATCH, WORD_BITS, bit_position};
use crate::{
    Value,
    mlkem_encoding_air::MlKemEncodingAir,
    mlkem_field::{Canonical, Compression},
};
use p3_field::PrimeCharacteristicRing;

pub(super) struct Prepared {
    pub coefficients: [u16; COEFFICIENTS],
    pub ciphertext: [u8; 1088],
}

impl Prepared {
    pub fn new(u: &[[u16; 256]; 3], v: &[u16; 256]) -> Option<Self> {
        let mut input = Self {
            coefficients: [0; COEFFICIENTS],
            ciphertext: [0; 1088],
        };
        let encoding = MlKemEncodingAir::new(10)?;
        for (index, polynomial) in u.iter().enumerate() {
            input.coefficients[index * 256..(index + 1) * 256].copy_from_slice(polynomial);
            input.ciphertext[index * 320..(index + 1) * 320]
                .copy_from_slice(&encoding.encode(*polynomial)?);
        }
        input.coefficients[768..].copy_from_slice(v);
        input.ciphertext[960..].copy_from_slice(&MlKemEncodingAir::new(4)?.encode(*v)?);
        Some(input)
    }

    pub fn write(&self, row: &mut [Value], index: usize) {
        for (target, &value) in row[..COEFFICIENTS].iter_mut().zip(&self.coefficients) {
            *target = Value::from_u16(value);
        }
        if index < 768 {
            arithmetic::<10>(row, self.coefficients[index]);
        } else {
            arithmetic::<4>(row, self.coefficients.get(index).copied().unwrap_or(0));
        }
        self.write_words(row, index);
    }

    fn write_words(&self, row: &mut [Value], index: usize) {
        let limb = if index < COEFFICIENTS {
            bit_position(index) / 16
        } else {
            usize::MAX
        };
        for word in 0..2 {
            let value = limb
                .checked_add(word)
                .map_or(0, |i| public_word(&self.ciphertext, i));
            for bit in 0..16 {
                row[WORD_BITS + word * 16 + bit] = Value::from_bool(value & (1 << bit) != 0);
            }
        }
    }
}

fn arithmetic<const D: usize>(row: &mut [Value], input: u16) {
    let value = Compression::<Value, D>::witness(input).expect("checked canonical input");
    canonical(row, SCRATCH, value.input);
    row[LOCAL_BITS..LOCAL_BITS + D].copy_from_slice(&value.compressed);
    row[ROLLOVER] = value.rollover;
    canonical(row, REMAINDER, value.remainder);
}

fn canonical(row: &mut [Value], start: usize, value: Canonical<Value>) {
    row[start] = value.value;
    row[start + 1..start + 13].copy_from_slice(&value.bits);
    row[start + 13..start + 25].copy_from_slice(&value.gap);
}

fn public_word(ciphertext: &[u8; 1088], index: usize) -> u16 {
    ciphertext
        .as_chunks::<2>()
        .0
        .get(index)
        .copied()
        .map(u16::from_le_bytes)
        .unwrap_or(0)
}
