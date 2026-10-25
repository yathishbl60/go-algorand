// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::Expected;

pub(super) fn check(slots: &[Expected<'_>]) -> Result<(), Error> {
    let main = &slots[0];
    let first = if main.kind == 1 { 128 } else { 64 };
    for output in &slots[1..] {
        context(output)?;
        commitment(main, output, first)?;
    }
    Ok(())
}

fn context(slot: &Expected<'_>) -> Result<(), Error> {
    let values = slot.values;
    if values[147] != u16::from(slot.role) {
        return Err(Error::PublicValues);
    }
    for (value, expected) in values[180..184].iter().zip(slot.output.to_be_bytes()) {
        if *value != u16::from(expected) {
            return Err(Error::PublicValues);
        }
    }
    Ok(())
}

fn commitment(main: &Expected<'_>, output: &Expected<'_>, first: usize) -> Result<(), Error> {
    let offset = first + output.output as usize * 32;
    let limbs = main
        .values
        .get(offset..offset + 32)
        .ok_or(Error::PublicValues)?;
    for (bytes, limb) in output.values[184..248].as_chunks::<2>().0.iter().zip(limbs) {
        if bytes[0] > 255 || bytes[1] > 255 || bytes[0] + (bytes[1] << 8) != *limb {
            return Err(Error::PublicValues);
        }
    }
    Ok(())
}
