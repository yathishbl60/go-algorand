// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    ADD_INDEX, ADD_SLOT, CODEC_FLAGS, DOT_FLAGS, FWD_FLAGS, FWD_SLOT, INV_FLAGS, INV_SLOT,
    RAW_CARRY,
    plan::{self, Step},
    schedule::Tables,
};
use crate::Value;
use p3_field::PrimeCharacteristicRing;

pub(super) fn write(row: &mut [Value], index: usize, tables: &Tables) {
    match plan::at(index) {
        Step::Forward(slot, local) => forward(row, slot, local, &tables.forward),
        Step::Dot(local) => {
            copy(row, 514, &tables.dot, local, 145);
            frame(row, DOT_FLAGS, local, 2048);
        }
        Step::Inverse(slot, local) => inverse(row, slot, local, &tables.inverse),
        Step::Addition(slot, local) => {
            row[ADD_SLOT + slot] = Value::ONE;
            row[ADD_INDEX + local] = Value::ONE;
            row[RAW_CARRY] = Value::from_bool(local < 255);
        }
        Step::Codec(local) => codec(row, local, &tables.codec),
    }
}

fn inverse(row: &mut [Value], slot: usize, local: usize, table: &[Value]) {
    copy(row, 0, table, local, 514);
    frame(row, INV_FLAGS, local, 2048);
    row[INV_SLOT + slot] = Value::ONE;
    row[RAW_CARRY] = Value::ONE;
}

fn frame(row: &mut [Value], start: usize, local: usize, rows: usize) {
    row[start] = Value::ONE;
    row[start + 1] = Value::from_bool(local == 0);
    row[start + 2] = Value::from_bool(local + 1 == rows);
    row[start + 3] = Value::from_bool(local + 1 < rows);
}

fn copy(row: &mut [Value], start: usize, source: &[Value], local: usize, width: usize) {
    row[start..start + width].copy_from_slice(&source[local * width..(local + 1) * width]);
}

fn forward(row: &mut [Value], slot: usize, local: usize, table: &[Value]) {
    copy(row, 0, table, local, 514);
    frame(row, FWD_FLAGS, local, 1024);
    row[FWD_SLOT + slot] = Value::ONE;
}

fn codec(row: &mut [Value], local: usize, table: &[Value]) {
    copy(row, 659, table, local, 1024);
    frame(row, CODEC_FLAGS, local, 2048);
}
