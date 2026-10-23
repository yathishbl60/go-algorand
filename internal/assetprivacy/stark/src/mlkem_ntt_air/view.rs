// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::mlkem_field::Canonical;

pub(super) fn canonical<T: Copy>(row: &[T], start: usize) -> Canonical<T> {
    Canonical {
        value: row[start],
        bits: row[start + 1..start + 13]
            .try_into()
            .expect("12 coefficient bits"),
        gap: row[start + 13..start + 25].try_into().expect("12 gap bits"),
    }
}
