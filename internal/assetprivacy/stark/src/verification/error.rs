// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Stable rejection categories; none grants native authorization.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    WireLimit,
    MemoryLimit,
    Truncated,
    NonCanonical,
    Shape,
    Artifact,
    PublicValues,
    InvalidProof,
    BackendPanic,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "native privacy candidate proof rejected: {self:?}")
    }
}

impl std::error::Error for Error {}
