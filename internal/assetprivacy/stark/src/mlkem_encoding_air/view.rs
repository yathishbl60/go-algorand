// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{LOCAL_BITS, REMAINDER, ROLLOVER};
use crate::mlkem_field::{Canonical, Compression};

pub(super) fn canonical<T: Copy>(row: &[T], start: usize) -> Canonical<T> {
    Canonical {
        value: row[start],
        bits: row[start + 1..start + 13].try_into().expect("12 bits"),
        gap: row[start + 13..start + 25].try_into().expect("12 gap bits"),
    }
}

pub(super) fn compression<T: Copy, const D: usize>(row: &[T]) -> Compression<T, D> {
    Compression {
        input: canonical(row, 0),
        compressed: row[LOCAL_BITS..LOCAL_BITS + D]
            .try_into()
            .expect("compression bits"),
        rollover: row[ROLLOVER],
        remainder: canonical(row, REMAINDER),
    }
}
