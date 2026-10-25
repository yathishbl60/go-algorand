// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub const ROWS: usize = 8192;
pub const PUBLIC_VALUES: usize = 96;
pub(super) const ACCOUNT: usize = 0;
pub(super) const VALUE: usize = 256;
pub(super) const SERIAL: usize = 320;
pub(super) const SECRET: usize = 576;
pub(super) const OWNER: usize = 832;
pub(super) const COMMIT: usize = 864;
pub(super) const INVERSE: usize = 896;
pub(super) const WORK: usize = 897;
pub(super) const SCRATCH: usize = 7350;
pub const WIDTH: usize = WORK + SCRATCH;
pub(super) const FLAGS: usize = crate::current_eligibility_air::PREPROCESSED;
pub const PREPROCESSED: usize = FLAGS + 20;
pub(super) const NOTE_ROWS: usize = 512;
pub(super) const OWNER_ROW: usize = NOTE_ROWS;
pub(super) const OWNER_ROWS: usize = 49;
pub(super) const NULLIFIER_ROW: usize = OWNER_ROW + OWNER_ROWS;
pub(super) const NULLIFIER_ROWS: usize = 97;
pub(super) const MEMBER_ROW: usize = NULLIFIER_ROW + NULLIFIER_ROWS;
pub(super) const ELIGIBILITY_ROW: usize = MEMBER_ROW + crate::membership_vertical::FRAME_ROWS;
pub const FRAME_ROWS: usize = ELIGIBILITY_ROW + crate::current_eligibility_air::FRAME_ROWS;
pub(super) const STAGES: [(usize, usize); 5] = [
    (0, NOTE_ROWS),
    (OWNER_ROW, OWNER_ROWS),
    (NULLIFIER_ROW, NULLIFIER_ROWS),
    (MEMBER_ROW, crate::membership_vertical::FRAME_ROWS),
    (ELIGIBILITY_ROW, crate::current_eligibility_air::FRAME_ROWS),
];

const _: () = assert!(FRAME_ROWS == 7380);
const _: () = assert!(FRAME_ROWS <= ROWS);
const _: () = assert!(crate::membership_vertical::WIDTH <= SCRATCH);
const _: () = assert!(crate::current_eligibility_air::WIDTH <= SCRATCH);
const _: () = assert!(crate::membership_vertical::PREPROCESSED <= FLAGS);
