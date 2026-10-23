// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::BITS;

pub(super) struct State {
    pub y: u128,
    pub x: u128,
    pub v: u128,
    pub z: u128,
    pub data: u128,
}

impl State {
    pub fn new() -> Self {
        Self {
            y: 0,
            x: 0,
            v: 0,
            z: 0,
            data: 0,
        }
    }

    pub fn start(&mut self, h: u128, data: u128) {
        self.data = data;
        self.x = self.y ^ data;
        self.v = h;
        self.z = 0;
    }

    pub fn input(&self, bit: usize) -> bool {
        self.x & (1 << (127 - bit)) != 0
    }

    pub fn advance(&mut self, bit: usize) {
        let selected = self.v & 0u128.wrapping_sub(u128::from(self.input(bit)));
        if bit == BITS - 1 {
            self.y = self.z ^ selected;
        } else {
            self.z ^= selected;
            self.v = (self.v >> 1) ^ ((0xe1u128 << 120) & 0u128.wrapping_sub(self.v & 1));
        }
    }
}
