// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MATRIX_ROW, NOISE_ROW, PKE_ROW, WIDTH, WORK};
use crate::{
    Value, mlkem_host::Encapsulation, mlkem_noise_air::MlKemNoiseAir, mlkem_pke_air::MlKemPkeAir,
    mlkem_stream_derivation_air::MlKemStreamDerivationAir,
    mlkem_stream_matrix_air::MlKemStreamMatrixAir,
};

pub(super) fn write(
    key: &[u8],
    message: [u8; 32],
    input: &Encapsulation,
    values: &mut [Value],
) -> Option<()> {
    MlKemStreamDerivationAir.write_into(key, message, values, (WIDTH, WORK, 0))?;
    matrices(input, values)?;
    noise(input, values)?;
    let ciphertext = MlKemPkeAir.write_into(&input.encryption, values, (WIDTH, 0, PKE_ROW))?;
    (ciphertext == input.encryption.ciphertext).then_some(())
}

fn matrices(input: &Encapsulation, values: &mut [Value]) -> Option<()> {
    for index in 0..9 {
        let air = MlKemStreamMatrixAir::new((index % 3) as u8, (index / 3) as u8)?;
        let frame = (WIDTH, WORK, MATRIX_ROW + 4096 * index);
        let output = air.write_into(input.encryption.matrix_seed, values, frame)?;
        if output != input.encryption.matrix[index / 3][index % 3] {
            return None;
        }
    }
    Some(())
}

fn noise(input: &Encapsulation, values: &mut [Value]) -> Option<()> {
    for index in 0..7 {
        MlKemNoiseAir::new(index as u8)?.write_into(
            input.coins,
            values,
            (WIDTH, WORK, NOISE_ROW + 512 * index),
        )?;
    }
    Some(())
}
