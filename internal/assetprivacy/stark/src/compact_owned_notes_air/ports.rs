// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{OwnedNotesAir, schema::*};
use p3_air::WindowAccess;
use p3_field::{PrimeCharacteristicRing, PrimeField64};
use p3_lookup::{Count, InteractionBuilder};

const BUS: &str = "Algorand/AssetPrivacy/OwnedNotePrivateWords1";

impl<F: PrimeField64> OwnedNotesAir<F> {
    pub(super) fn eval_ports<AB: InteractionBuilder<F = F>>(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.current_slice();
        self.hash_port(builder, row);
        let owner: Vec<AB::Expr> = row[OWNER..OWNER + 32].iter().map(|&v| v.into()).collect();
        let note = row[RELAY + 5].into();
        let head = row[RELAY + HEAD].into();
        emit(builder, tuple(1, note, AB::Expr::ZERO, owner), -head);
        self.word_ports(builder, row);
    }

    fn hash_port<AB: InteractionBuilder<F = F>>(&self, builder: &mut AB, row: &[AB::Var]) {
        let program = &row[RELAY + 9..RELAY + 20];
        let words = (0..32)
            .map(|word| {
                self.hash
                    .output_word::<AB::Expr, _>(&row[HASH..HASH + 1920], program, word)
                    .expect("fixed state and program width")
            })
            .collect();
        let active = AB::Expr::ONE - program[0] - program[1];
        emit(
            builder,
            tuple(1, program[2].into(), AB::Expr::ZERO, words),
            active,
        );
    }

    fn word_ports<AB: InteractionBuilder<F = F>>(&self, builder: &mut AB, row: &[AB::Var]) {
        let (note, word) = (row[RELAY + 5].into(), row[RELAY + WORD_INDEX].into());
        let active: AB::Expr = row[RELAY + WORD_ACTIVE].into();
        let value = self
            .note
            .overlapping_word::<AB::Expr, _>(&row[NOTE..NOTE + 11])
            .expect("fixed note width");
        emit(
            builder,
            tuple(2, note.clone(), word.clone(), vec![row[SELECTED].into()]),
            active.clone(),
        );
        emit(builder, tuple(2, note, word, vec![value]), -active);
    }
}

fn tuple<E: PrimeCharacteristicRing>(kind: u8, note: E, word: E, values: Vec<E>) -> Vec<E> {
    let mut fields = vec![E::from_u8(kind), note, word];
    fields.extend(values);
    fields.resize(35, E::ZERO);
    fields
}

fn emit<AB: InteractionBuilder>(builder: &mut AB, fields: Vec<AB::Expr>, count: AB::Expr) {
    builder.push_interaction(BUS, fields, Count::bounded(count, 1));
}
