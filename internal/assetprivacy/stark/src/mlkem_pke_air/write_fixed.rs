// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DOT, FINAL, MESSAGE, NOISE, RHAT, ROWS, WIDTH, prepared::Prepared};
use crate::{Value, mlkem_host::Polynomial};
use p3_field::PrimeCharacteristicRing;

pub(super) fn write(values: &mut [Value], input: &Prepared<'_>, shape: (usize, usize, usize)) {
    let (stride, start, first) = shape;
    for parent in values.chunks_exact_mut(stride).skip(first).take(ROWS) {
        write_row(&mut parent[start..start + WIDTH], input);
    }
}

fn write_row(row: &mut [Value], input: &Prepared<'_>) {
    for (index, matrix_row) in input.input.matrix.iter().enumerate() {
        polynomials(row, index * 768, matrix_row);
    }
    polynomials(row, 2304, &input.input.public_polynomials);
    polynomials(row, RHAT, &input.dot_input.r_hat);
    polynomials(row, DOT, &input.dots);
    polynomials(row, NOISE, &input.input.noise);
    polynomials(row, FINAL, &input.input.u);
    polynomials(row, FINAL + 768, &[input.input.v]);
    for (index, &coefficient) in input.input.message.iter().enumerate() {
        row[MESSAGE + index] = Value::from_bool(coefficient == 1665);
    }
}

fn polynomials(row: &mut [Value], start: usize, source: &[Polynomial]) {
    for (index, polynomial) in source.iter().enumerate() {
        for (coefficient, &value) in polynomial.iter().enumerate() {
            row[start + index * 256 + coefficient] = Value::from_u16(value);
        }
    }
}
