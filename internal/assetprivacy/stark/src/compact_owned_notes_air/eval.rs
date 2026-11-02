// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{OwnedNotesAir, schema::*};
use crate::relay_builder::RelayBuilder;
use p3_air::{Air, AirBuilder, WindowAccess};
use p3_field::PrimeField64;
use p3_lookup::InteractionBuilder;

impl<F: PrimeField64, AB: InteractionBuilder<F = F>> Air<AB> for OwnedNotesAir<F> {
    fn eval(&self, builder: &mut AB) {
        self.eval_context(builder);
        self.eval_relays(builder);
        self.note.eval(&mut RelayBuilder::new(
            builder,
            NOTE..NOTE + 11,
            RELAY..RELAY + 9,
        ));
        self.hash.eval(&mut RelayBuilder::new(
            builder,
            HASH..HASH + 1920,
            RELAY + 9..RELAY + 20,
        ));
        self.eval_bridge(builder);
        self.eval_private_input(builder);
        self.eval_ports(builder);
    }
}

impl<F: PrimeField64> OwnedNotesAir<F> {
    fn eval_context<AB: AirBuilder<F = F>>(&self, builder: &mut AB) {
        for index in 0..64 {
            let value = builder.public_values()[index];
            builder.assert_eq(value, self.public_values[index]);
        }
    }

    fn eval_relays<AB: AirBuilder<F = F>>(&self, builder: &mut AB) {
        let main = builder.main();
        for index in 0..RELAY_COUNT {
            let value = builder.periodic_values()[index];
            builder.assert_eq(main.current_slice()[RELAY + index], value);
        }
    }
}
