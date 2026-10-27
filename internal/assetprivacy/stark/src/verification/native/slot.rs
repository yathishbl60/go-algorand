// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{Error, selection::Layout};
use super::cursor::Cursor;

pub(super) struct Slot {
    pub layout: Layout,
    pub output: u32,
    pub role: u8,
    pub values: Vec<u16>,
}

impl Slot {
    pub(super) fn parse(cursor: &mut Cursor<'_>) -> Result<Self, Error> {
        let layout = Layout::from_byte(cursor.byte()?).ok_or(Error::Shape)?;
        let output = cursor.u32()?;
        let role = cursor.byte()?;
        let values = values(cursor, layout.public_count())?;
        Ok(Self {
            layout,
            output,
            role,
            values,
        })
    }

    pub(super) fn amount(&self, start: usize) -> Result<u64, Error> {
        let words = self
            .values
            .get(start..start + 4)
            .ok_or(Error::PublicValues)?;
        Ok(words.iter().enumerate().fold(0, |value, (limb, word)| {
            value | (u64::from(*word) << (16 * limb))
        }))
    }

    pub(super) fn bytes<const N: usize>(&self, start: usize) -> Result<[u8; N], Error> {
        let cells = self
            .values
            .get(start..start + N)
            .ok_or(Error::PublicValues)?;
        let mut bytes = [0; N];
        for (byte, cell) in bytes.iter_mut().zip(cells) {
            *byte = u8::try_from(*cell).map_err(|_| Error::PublicValues)?;
        }
        Ok(bytes)
    }
}

fn values(cursor: &mut Cursor<'_>, expected: usize) -> Result<Vec<u16>, Error> {
    let count = usize::from(cursor.u16()?);
    if count != expected {
        return Err(Error::PublicValues);
    }
    let bytes = cursor.take(count * 2)?;
    let mut values = Vec::new();
    values
        .try_reserve_exact(count)
        .map_err(|_| Error::AllocationFailure)?;
    for cell in bytes.as_chunks::<2>().0 {
        values.push(u16::from_le_bytes(*cell));
    }
    Ok(values)
}
