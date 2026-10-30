// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{COEFFICIENTS, MlKemKeygenNoiseAir, NOISE_WIDTH, RHO, SEED_WIDTH, SIGMA, WORK};
use crate::{Value, mlkem_keygen_seed_air};
use p3_air::{AirBuilder, WindowAccess};

impl MlKemKeygenNoiseAir {
    pub(super) fn eval_constants<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        for (column, &current) in row[..WORK].iter().enumerate() {
            builder
                .when_transition()
                .assert_eq(current, main.next_slice()[column]);
        }
        for &bit in &row[..COEFFICIENTS] {
            builder.assert_bool(bit);
        }
    }
    pub(super) fn eval_seed<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        let child =
            mlkem_keygen_seed_air::wires(&row[WORK..WORK + SEED_WIDTH]).expect("seed width");
        let active = builder.preprocessed().current_slice()[0];
        let roots = [&row[..RHO], &row[RHO..SIGMA], &row[SIGMA..COEFFICIENTS]];
        for (root, bits) in roots.into_iter().zip([child.seed, child.rho, child.sigma]) {
            for (&root, &bit) in root.iter().zip(bits) {
                builder.when(active).assert_eq(root, bit);
            }
        }
    }
    pub(super) fn eval_noise<AB: AirBuilder<F = Value>>(&self, builder: &mut AB) {
        for i in 0..6 {
            self.eval_polynomial(builder, i);
        }
    }
    fn eval_polynomial<AB: AirBuilder<F = Value>>(&self, builder: &mut AB, i: usize) {
        let main = builder.main();
        let row = main.current_slice();
        let child = &row[WORK..WORK + NOISE_WIDTH];
        let active = builder.preprocessed().current_slice()[4 * (i + 1)];
        for bit in 0..256 {
            let source = self.noise[i]
                .seed_bit_column(bit / 8, bit % 8)
                .expect("seed bit");
            builder
                .when(active)
                .assert_eq(row[SIGMA + bit], child[source]);
        }
        let coefficients = self.noise[i].coefficients(child).expect("noise width");
        for (j, &value) in coefficients.iter().enumerate() {
            builder
                .when(active)
                .assert_eq(row[COEFFICIENTS + i * 256 + j], value);
        }
    }
}
