// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MATRIX_ROW, NOISE_ROW, PKE_ROW, WORK, target::shift};
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
    target: (usize, usize, usize),
) -> Option<()> {
    MlKemStreamDerivationAir.write_into(key, message, values, shift(target, WORK, 0))?;
    matrices(input, values, target)?;
    noise(input, values, target)?;
    let ciphertext =
        MlKemPkeAir.write_into(&input.encryption, values, shift(target, 0, PKE_ROW))?;
    (ciphertext == input.encryption.ciphertext).then_some(())
}

fn matrices(
    input: &Encapsulation,
    values: &mut [Value],
    target: (usize, usize, usize),
) -> Option<()> {
    for index in 0..9 {
        let air = MlKemStreamMatrixAir::new((index % 3) as u8, (index / 3) as u8)?;
        let frame = shift(target, WORK, MATRIX_ROW + 4096 * index);
        let output = air.write_into(input.encryption.matrix_seed, values, frame)?;
        if output != input.encryption.matrix[index / 3][index % 3] {
            return None;
        }
    }
    Some(())
}

fn noise(input: &Encapsulation, values: &mut [Value], target: (usize, usize, usize)) -> Option<()> {
    for index in 0..7 {
        let frame = shift(target, WORK, NOISE_ROW + 512 * index);
        MlKemNoiseAir::new(index as u8)?.write_into(input.coins, values, frame)?;
    }
    Some(())
}
