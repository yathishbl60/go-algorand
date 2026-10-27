// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::{HEADER, binding, cursor::Cursor, header::Header, slot::Slot};

pub(super) struct Request {
    pub header: Header,
    pub slots: Vec<Slot>,
}

impl Request {
    pub(super) fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let header = Header::parse(bytes)?;
        let mut cursor = Cursor::new(&bytes[HEADER..]);
        let mut slots = Vec::new();
        slots
            .try_reserve_exact(header.count)
            .map_err(|_| Error::AllocationFailure)?;
        for _ in 0..header.count {
            slots.push(Slot::parse(&mut cursor)?);
        }
        cursor.finish()?;
        binding::main(&header.context, &slots[0])?;
        Ok(Self { header, slots })
    }
}
