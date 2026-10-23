// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Two actual private inputs and a public credit; no dummy private outputs.
//! Current controls, root freshness and native public credit remain separate.

mod binding;
mod public;
mod trace;

pub use crate::transfer_air::{InputPublic, InputWitness};
pub use public::PublicStatement;

use crate::{Value, member_note_air::MemberNoteAir};
use p3_air::{Air, AirBuilder, BaseAir};

pub const PUBLIC_VALUES: usize = 164;
const CREDIT_START: usize = 128;
const CONTEXT_START: usize = 132;

#[derive(Clone)]
pub struct UnshieldAir {
    input: MemberNoteAir,
    credit: u64,
}

impl UnshieldAir {
    pub fn new(network: [u8; 32], asset: u64, policy: [u8; 64], credit: u64) -> Option<Self> {
        Some(Self {
            input: MemberNoteAir::new(network, asset, policy)?,
            credit,
        })
    }
    fn auxiliary_start(&self) -> usize {
        2 * self.input.width()
    }
    fn width(&self) -> usize {
        self.auxiliary_start() + 4
    }
}

impl BaseAir<Value> for UnshieldAir {
    fn width(&self) -> usize {
        self.width()
    }
    fn num_public_values(&self) -> usize {
        PUBLIC_VALUES
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for UnshieldAir {
    fn eval(&self, builder: &mut AB) {
        for index in 0..2 {
            self.eval_input(builder, index);
        }
        self.eval_conservation(builder);
        self.eval_credit(builder);
    }
}
