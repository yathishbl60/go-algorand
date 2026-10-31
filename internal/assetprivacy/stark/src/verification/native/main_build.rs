// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{
    Error,
    selection::{Layout, Registry},
};
use super::{header::Context, slot::Slot, target::Target};

pub(super) fn build(context: &Context, registry: &Registry, slot: &Slot) -> Result<Target, Error> {
    match slot.layout {
        Layout::Shield => super::shield::build(context, registry, slot),
        Layout::Transfer | Layout::Unshield => super::main::spend(context, registry, slot),
        Layout::ConfidentialTransfer
        | Layout::ConfidentialUnshield
        | Layout::ConfidentialShield => super::confidential::build(context, registry, slot),
        _ => super::main::recovery(context, registry, slot),
    }
}
