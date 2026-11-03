// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Local input preparation failures; never a proof-verification success signal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputError {
    Shape,
    Limits,
    Allocation,
    RandomState,
}

impl core::fmt::Display for InputError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(match self {
            Self::Shape => "invalid streamed input geometry",
            Self::Limits => "streamed input scalar budget exceeded",
            Self::Allocation => "streamed input allocation failed",
            Self::RandomState => "streamed input randomness state unavailable",
        })
    }
}

impl std::error::Error for InputError {}
