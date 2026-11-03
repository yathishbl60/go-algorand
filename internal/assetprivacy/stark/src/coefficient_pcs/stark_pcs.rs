// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Challenger, CoefficientPcs, Dense, Domain, E, Error, ProverData, V};
use super::{evaluations, import, opening, quotient, randomization};
use crate::compact_storage::TreeCap;
use p3_commit::{OpenedValues, OpeningRequest, UnivariateStarkPcs};

impl UnivariateStarkPcs<E, Challenger> for CoefficientPcs {
    type EvaluationsOnDomain<'a> = Dense;
    const ZK: bool = true;

    fn log_max_trace_height(&self) -> usize {
        16
    }
    fn log_min_trace_height(&self) -> usize {
        9
    }

    fn commit_preprocessing(
        &self,
        _: impl IntoIterator<Item = (Domain, Dense)>,
    ) -> Result<(TreeCap, ProverData), Error> {
        Err(Error::Unsupported)
    }

    fn get_quotient_ldes(
        &self,
        evaluations: impl IntoIterator<Item = (Domain, Dense)>,
        chunks: usize,
    ) -> Result<Vec<Dense>, Error> {
        quotient::ldes(self, evaluations.into_iter().collect(), chunks)
    }

    fn commit_ldes(&self, ldes: Vec<Dense>) -> Result<(TreeCap, ProverData), Error> {
        import::ldes(self, ldes)
    }

    fn get_evaluations_on_domain(&self, data: &ProverData, index: usize, domain: Domain) -> Dense {
        evaluations::evaluate(self, data, index, domain, false)
            .expect("coefficient PCS evaluation shape or resource limit exceeded")
    }

    fn get_evaluations_on_domain_no_random(
        &self,
        data: &ProverData,
        index: usize,
        domain: Domain,
    ) -> Dense {
        evaluations::evaluate(self, data, index, domain, true)
            .expect("coefficient PCS evaluation shape or resource limit exceeded")
    }

    fn open_with_preprocessing(
        &self,
        rounds: Vec<OpeningRequest<'_, ProverData, E>>,
        challenger: &mut Challenger,
        preprocessing: Option<usize>,
    ) -> Result<(OpenedValues<E>, Self::Proof), Error> {
        opening::open(self, rounds, challenger, preprocessing)
    }

    fn verify_with_preprocessing(
        &self,
        rounds: Vec<p3_commit::CommitmentOpening<E, TreeCap, Domain>>,
        proof: &Self::Proof,
        challenger: &mut Challenger,
        preprocessing: Option<usize>,
    ) -> Result<(), Self::Error> {
        self.stock
            .verify_with_preprocessing(rounds, proof, challenger, preprocessing)
    }

    fn get_opt_randomization_poly_commitment(
        &self,
        domains: impl IntoIterator<Item = Domain>,
    ) -> Result<Option<(TreeCap, ProverData)>, Error> {
        randomization::commit(self, domains.into_iter().collect()).map(Some)
    }

    fn build_periodic_lde_table(
        &self,
        columns: &[Vec<V>],
        trace: Domain,
        quotient: Domain,
    ) -> p3_commit::PeriodicLdeTable<V> {
        super::stark_verify::periodics(self, columns, trace, quotient)
    }
}
