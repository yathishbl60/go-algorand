// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::{Expected, MAIN_OUTPUT, MAX_SLOTS};

pub(super) fn check(slots: &[Expected<'_>]) -> Result<(), Error> {
    if slots.is_empty() || slots.len() > MAX_SLOTS {
        return Err(Error::Shape);
    }
    let main = &slots[0];
    if main.output != MAIN_OUTPUT || main.role != 0 {
        return Err(Error::Shape);
    }
    let (count, recipient, retained) = operation(main.kind)?;
    let mut rest = &slots[1..];
    for ordinal in 0..count {
        let kind = recipient_kind(ordinal, recipient, retained);
        rest = output(rest, ordinal, kind)?;
    }
    if !rest.is_empty() || !same_registry(slots, main) {
        return Err(Error::Shape);
    }
    super::binding::check(slots)
}

fn same_registry(slots: &[Expected<'_>], main: &Expected<'_>) -> bool {
    slots
        .iter()
        .all(|s| s.profile_id() == main.profile_id() && s.registry_id() == main.registry_id())
}

fn recipient_kind(ordinal: u32, recipient: u8, retained: Option<u32>) -> u8 {
    if retained == Some(ordinal) {
        19
    } else {
        recipient
    }
}

fn operation(kind: u8) -> Result<(u32, u8, Option<u32>), Error> {
    match kind {
        1 | 3 => Ok((2, 17, None)),
        2 | 7 | 21 | 22 => Ok((0, 0, None)),
        4 => Ok((2, 20, Some(1))),
        5 => Ok((1, 20, None)),
        6 => Ok((1, 20, Some(0))),
        _ => Err(Error::Shape),
    }
}

fn output<'a, 'b>(
    slots: &'b [Expected<'a>],
    ordinal: u32,
    kind: u8,
) -> Result<&'b [Expected<'a>], Error> {
    first_output(slots, ordinal, kind)?;
    let mut consumed = 1;
    let mut role = 1;
    for slot in &slots[1..] {
        if slot.output != ordinal {
            break;
        }
        if slot.kind != 18 || slot.role <= role {
            return Err(Error::Shape);
        }
        role = slot.role;
        consumed += 1;
    }
    Ok(&slots[consumed..])
}

fn first_output(slots: &[Expected<'_>], ordinal: u32, kind: u8) -> Result<(), Error> {
    if slots.first().ok_or(Error::Shape)?.descriptor() != (kind, ordinal, 1) {
        return Err(Error::Shape);
    }
    Ok(())
}
