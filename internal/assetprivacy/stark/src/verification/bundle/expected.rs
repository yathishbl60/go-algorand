// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;
use super::{MAIN_OUTPUT, target::Target};

/// Borrowed immutable expected statement. No proof bytes can select its AIR type.
#[derive(Clone, Copy)]
pub struct Expected<'a> {
    pub(super) kind: u8,
    pub(super) output: u32,
    pub(super) role: u8,
    pub(super) target: Target<'a>,
    pub(super) values: &'a [u16],
}

impl<'a> Expected<'a> {
    pub(super) fn new(
        kind: u8,
        output: u32,
        role: u8,
        target: Target<'a>,
        values: &'a [u16],
    ) -> Result<Self, Error> {
        if values.len() != public_count(kind) || values.len() != target.public_len() {
            return Err(Error::PublicValues);
        }
        Ok(Self {
            kind,
            output,
            role,
            target,
            values,
        })
    }

    pub(super) fn main(kind: u8, target: Target<'a>, values: &'a [u16]) -> Result<Self, Error> {
        Self::new(kind, MAIN_OUTPUT, 0, target, values)
    }

    pub fn descriptor(&self) -> (u8, u32, u8) {
        (self.kind, self.output, self.role)
    }
    pub fn profile_id(&self) -> [u8; 64] {
        self.target.profile()
    }
    pub fn registry_id(&self) -> [u8; 64] {
        self.target.registry()
    }
}

fn public_count(kind: u8) -> usize {
    match kind {
        1 => 264,
        2 => 196,
        3 => 100,
        8 => 392,
        9 => 324,
        10 => 228,
        21 => 64,
        22 => 1216,
        4 => 192,
        5 => 160,
        6 => 164,
        7 => 132,
        17 | 18 => 1132,
        19 | 20 => 1100,
        _ => 0,
    }
}
