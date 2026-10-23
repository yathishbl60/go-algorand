// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private ownership, serial-derived nullifier, and 32-level membership.
//! Public values are 32 nullifier limbs followed by 32 root limbs.
//! Transfer conservation, ciphertexts, group binding and ledger freshness are separate.

mod binding;
mod builder;
mod trace;
mod value;

use crate::prefix_builder::PrefixBuilder;
use crate::{Value, membership_air::MembershipAir, nullified_note_air::NullifiedNoteAir};
use builder::RootBuilder;
use p3_air::{Air, AirBuilder, BaseAir};

#[derive(Clone)]
pub struct MemberNoteAir {
    note: NullifiedNoteAir,
    membership: MembershipAir,
}

impl MemberNoteAir {
    pub fn new(network: [u8; 32], asset: u64, policy: [u8; 64]) -> Option<Self> {
        Some(Self {
            note: NullifiedNoteAir::new(network, asset, policy)?,
            membership: MembershipAir::new(network, asset, policy)?,
        })
    }
    pub(crate) fn width(&self) -> usize {
        self.note.width() + self.membership.width()
    }
}

impl BaseAir<Value> for MemberNoteAir {
    fn width(&self) -> usize {
        self.width()
    }
    fn num_public_values(&self) -> usize {
        64
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for MemberNoteAir {
    fn eval(&self, builder: &mut AB) {
        self.note.eval(&mut PrefixBuilder {
            inner: builder,
            width: self.note.width(),
        });
        let window = builder.main();
        self.membership.eval(&mut RootBuilder {
            inner: builder,
            window,
            start: self.note.width(),
        });
        self.eval_commitment(builder);
    }
}
