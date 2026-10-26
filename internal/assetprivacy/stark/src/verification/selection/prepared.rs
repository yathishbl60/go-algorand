// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Error, implementation, shape::Shape};
use super::Layout;
use crate::Value;
use p3_air::BaseAir;
use sha3::{Digest, Sha3_512};

/// Exact AIR plus canonical identity. Construct only through the typed factories.
#[derive(Clone)]
pub struct Prepared<A> {
    pub(super) air: A,
    pub(super) layout: Layout,
    pub(super) relation: [u8; 64],
}

impl<A: BaseAir<Value>> Prepared<A> {
    pub(super) fn new(air: A, layout: Layout, args: &[u8]) -> Result<Self, Error> {
        if Shape::from_air(&air)? != layout.shape() {
            return Err(Error::Shape);
        }
        Ok(Self {
            air,
            layout,
            relation: relation_id(layout, args),
        })
    }

    pub fn air(&self) -> &A {
        &self.air
    }
    pub fn layout(&self) -> Layout {
        self.layout
    }
    pub fn relation_id(&self) -> [u8; 64] {
        self.relation
    }
}

fn relation_id(layout: Layout, args: &[u8]) -> [u8; 64] {
    let mut hash = Sha3_512::new();
    hash.update(b"Algorand/AssetPrivacy/Candidate1/RelationV1/NullifierV2/");
    hash.update(implementation::ID);
    hash.update([layout as u8]);
    hash.update((args.len() as u32).to_le_bytes());
    hash.update(args);
    hash.finalize().into()
}
