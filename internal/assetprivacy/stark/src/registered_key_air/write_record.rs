// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    KeyMaterial, RegisteredKeyAir, RegisteredKeyInput, prepared::Prepared, record::RecordAir,
};
use crate::{Value, spend_key_air::SpendKeyAir, view_key_air::ViewKeyAir};

pub(super) fn write(
    air: &RegisteredKeyAir,
    input: &Prepared<'_>,
    values: &mut [Value],
    shape: (usize, usize, usize),
) -> Option<()> {
    let source = input.input;
    let digest = match (&air.record, source.material) {
        (RecordAir::View(air), KeyMaterial::View(key)) => view(air, key, source, values, shape)?,
        (RecordAir::Spend(air), KeyMaterial::Spend(owner)) => {
            spend(air, owner, source, values, shape)?
        }
        _ => return None,
    };
    (digest == input.leaf).then_some(())
}

fn view(
    air: &ViewKeyAir,
    key: &[u8],
    source: RegisteredKeyInput<'_>,
    values: &mut [Value],
    shape: (usize, usize, usize),
) -> Option<[u8; 64]> {
    air.write_into(
        key,
        source.account,
        source.epoch,
        source.created,
        values,
        shape,
    )
}

fn spend(
    air: &SpendKeyAir,
    owner: [u8; 64],
    source: RegisteredKeyInput<'_>,
    values: &mut [Value],
    shape: (usize, usize, usize),
) -> Option<[u8; 64]> {
    air.write_into(
        owner,
        source.account,
        source.epoch,
        source.created,
        values,
        shape,
    )
}
