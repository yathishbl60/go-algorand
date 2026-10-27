// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Error, shape::Shape};
use super::{Layout, Registry};
use crate::Value;
use p3_air::BaseAir;
use sha3::{Digest, Sha3_512};

/// Exact AIR plus canonical identity. Construct only through the typed factories.
#[derive(Clone)]
pub struct Prepared<A> {
    pub(super) air: A,
    pub(super) layout: Layout,
    args: Vec<u8>,
}

impl<A: BaseAir<Value>> Prepared<A> {
    pub(super) fn new(air: A, layout: Layout, args: &[u8]) -> Result<Self, Error> {
        if Shape::from_air(&air)? != layout.shape() {
            return Err(Error::Shape);
        }
        Ok(Self {
            air,
            layout,
            args: args.to_vec(),
        })
    }

    pub fn air(&self) -> &A {
        &self.air
    }
    pub fn layout(&self) -> Layout {
        self.layout
    }
    pub fn relation_id(&self, registry: &Registry) -> [u8; 64] {
        relation_id(registry.reference_id(), self.layout, &self.args)
    }
}

fn relation_id(reference: [u8; 64], layout: Layout, args: &[u8]) -> [u8; 64] {
    let mut hash = Sha3_512::new();
    hash.update(b"Algorand/AssetPrivacy/Candidate1/RelationV1/NullifierV2/");
    hash.update(reference);
    hash.update([layout as u8]);
    hash.update((args.len() as u32).to_le_bytes());
    hash.update(args);
    hash.finalize().into()
}
