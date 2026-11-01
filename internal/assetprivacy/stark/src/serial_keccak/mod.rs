// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Private Keccak-f stages, constrained through adjacent local trace rows.
//! The containing AIR must bind the trusted schedule, absorption and terminals.
mod chi;
mod constants;
mod index;
mod native;
mod parity;
mod rho_pi;
mod theta;

pub(crate) use constants::{RC_POSITIONS, ROTATIONS, ROUND_CONSTANTS};
pub(crate) use index::{parity_index, rho_source, state_index};
pub(crate) use native::{chi_iota, rho_pi, theta};

use crate::Value;
use p3_air::{AirBuilder, WindowAccess};

pub(crate) const STATE_BITS: usize = 1600;
pub(crate) const PARITY_BITS: usize = 320;
pub(crate) const WIDTH: usize = STATE_BITS + PARITY_BITS;
pub(crate) const PREPROCESSED: usize = 11;

/// Selectors are Theta, RhoPi, ChiIota, absorption, then seven Iota bits.
/// Hold, absorption, initialization, final digest and group binding are external.
pub(crate) fn eval<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    main_offset: usize,
    pre_offset: usize,
) {
    let main = builder.main();
    for &bit in &main.current_slice()[main_offset..main_offset + STATE_BITS] {
        builder.assert_bool(bit);
    }
    parity::eval(builder, main_offset);
    theta::eval(builder, main_offset, pre_offset);
    rho_pi::eval(builder, main_offset, pre_offset);
    chi::eval(builder, main_offset, pre_offset);
}
