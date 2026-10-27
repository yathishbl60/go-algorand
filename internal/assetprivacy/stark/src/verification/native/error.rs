// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::Error;

/// Native context is trusted caller state; its failures are never bad-peer evidence.
#[derive(Debug)]
pub enum Failure {
    Request(Error),
    Proof(Error),
}

impl core::fmt::Display for Failure {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "native privacy verification failed: {self:?}")
    }
}

impl std::error::Error for Failure {}
