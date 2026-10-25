// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Fixed candidate envelope encryption bound to a canonical committed opening.
//! Current registry authority and mandatory disclosure policy remain separate.
mod bind_aad;
mod bind_gcm;
mod bind_hkdf;
mod bind_kem;
mod bind_note;
mod binding;
mod components;
mod context;
mod context_fields;
mod fixed;
mod frame_writer;
mod frames;
mod layout;
mod params;
mod prepared;
mod schedule;
mod schedule_copy;
mod target;
mod trace;
mod wire;
mod writer;
mod writer_aad;

use crate::{Value, envelope_aad_air::EnvelopeAadAir, hkdf_air::HkdfAir};
pub use context::EnvelopeContext;
pub use context_fields::EnvelopeFields;
pub use layout::{Wires, wires};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_matrix::dense::RowMajorMatrix;
pub(crate) use params::FRAME_ROWS;
use params::*;
pub use params::{PREPROCESSED, PUBLIC_VALUES, ROWS, WIDTH};
pub use trace::{EnvelopeInputs, EnvelopeWitness};

#[derive(Clone)]
pub struct EnvelopeAir {
    context: EnvelopeContext,
    aad: EnvelopeAadAir,
    hkdf: HkdfAir,
    note: crate::note_air::NoteCommitmentAir,
}

impl EnvelopeAir {
    pub fn new(context: EnvelopeContext) -> Option<Self> {
        let hkdf = HkdfAir::new()?;
        let note = context.note()?;
        if hkdf.width() != WORK_WIDTH || note.width() > WORK_WIDTH {
            return None;
        }
        Some(Self {
            context,
            aad: EnvelopeAadAir::new(*context.bytes()),
            hkdf,
            note,
        })
    }
}

impl BaseAir<Value> for EnvelopeAir {
    fn width(&self) -> usize {
        WIDTH
    }
    fn num_public_values(&self) -> usize {
        PUBLIC_VALUES
    }
    fn preprocessed_width(&self) -> usize {
        PREPROCESSED
    }
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<Value>> {
        Some(schedule::build(self))
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for EnvelopeAir {
    fn eval(&self, builder: &mut AB) {
        self.eval_components(builder);
        self.eval_constants(builder);
        self.eval_epoch(builder);
        self.eval_context(builder);
        self.eval_kem_binding(builder);
        self.eval_aad_binding(builder);
        self.eval_hkdf_binding(builder);
        self.eval_gcm_binding(builder);
        self.eval_note_binding(builder);
    }
}
