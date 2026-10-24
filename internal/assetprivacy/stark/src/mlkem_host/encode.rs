// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Polynomial;
use crate::mlkem_encoding_air::MlKemEncodingAir;

pub(super) fn ciphertext(u: &[Polynomial; 3], v: Polynomial) -> Option<[u8; 1088]> {
    let mut output = [0; 1088];
    let encoding = MlKemEncodingAir::new(10)?;
    for (index, &polynomial) in u.iter().enumerate() {
        let bytes = encoding.encode(polynomial)?;
        output[index * 320..(index + 1) * 320].copy_from_slice(&bytes);
    }
    let final_bytes = MlKemEncodingAir::new(4)?.encode(v)?;
    output[960..].copy_from_slice(&final_bytes);
    Some(output)
}
