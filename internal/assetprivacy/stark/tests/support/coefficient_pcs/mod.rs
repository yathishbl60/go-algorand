// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

mod budgets;
pub mod concurrent;
pub mod direct;
mod fixture;
pub mod guards;
pub mod notes;
#[path = "../owned_notes/fixture.rs"]
mod notes_fixture;
mod preprocessing;
mod rounds;

use algorand_asset_privacy_stark::{
    coefficient_pcs, compact, compact_owned_notes_air::OwnedNotesAir,
};
use p3_commit::Pcs;
use p3_field::coset::TwoAdicMultiplicativeCoset;
use p3_uni_stark::StarkGenericConfig;

type V = compact::Value32;
type E = compact::Challenge32;
type Air = OwnedNotesAir<V>;
type Config = coefficient_pcs::Config;
type Challenger = <Config as StarkGenericConfig>::Challenger;
type Cap = <coefficient_pcs::CoefficientPcs as Pcs<E, Challenger>>::Commitment;
type Proof = <coefficient_pcs::CoefficientPcs as Pcs<E, Challenger>>::Proof;
type Domain = TwoAdicMultiplicativeCoset<V>;
type Data = coefficient_pcs::ProverData;
type Points = Vec<Vec<Vec<E>>>;
type Claims = Vec<Vec<Vec<Vec<E>>>>;
type Round = p3_commit::CommitmentOpening<E, Cap, Domain>;
