// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    Challenger, CoefficientPcs, Dense, Domain, E, Error, ProverData, StockPcs, StockProof,
};
use super::{VerifierError, commit, opening};
use crate::compact_storage::TreeCap;
use p3_commit::{CommitmentOpening, OpenedValues, OpeningRequest, Pcs};

impl Pcs<E, Challenger> for CoefficientPcs {
    type Domain = Domain;
    type Commitment = TreeCap;
    type ProverData = ProverData;
    type Proof = StockProof;
    type Error = VerifierError;
    type ProverError = Error;

    fn natural_domain_for_degree(&self, degree: usize) -> Domain {
        <StockPcs as Pcs<E, Challenger>>::natural_domain_for_degree(&self.stock, degree)
    }

    fn commit(
        &self,
        evaluations: impl IntoIterator<Item = (Domain, Dense)>,
    ) -> Result<(TreeCap, ProverData), Error> {
        commit::regular(self, evaluations.into_iter().collect())
    }

    fn open(
        &self,
        rounds: Vec<OpeningRequest<'_, ProverData, E>>,
        challenger: &mut Challenger,
    ) -> Result<(OpenedValues<E>, StockProof), Error> {
        opening::open(self, rounds, challenger, None)
    }

    fn verify(
        &self,
        rounds: Vec<CommitmentOpening<E, TreeCap, Domain>>,
        proof: &StockProof,
        challenger: &mut Challenger,
    ) -> Result<(), VerifierError> {
        self.stock.verify(rounds, proof, challenger)
    }
}
