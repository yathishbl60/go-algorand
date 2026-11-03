//! Optional quotient evaluation scheduling; proof and verifier formats are unchanged.

use crate::config::{Challenge, Domain, PcsProverError, StarkGenericConfig as SGC};
use alloc::vec::Vec;
use p3_air::symbolic::AirLayout;
use p3_commit::Pcs;

/// A committed matrix and its index; the prover retains ownership of its data.
pub type CommittedMatrix<'a, SC> = (
    &'a <<SC as SGC>::Pcs as Pcs<<SC as SGC>::Challenge, <SC as SGC>::Challenger>>::ProverData,
    usize,
);

/// A fallible call to the existing PCS evaluation and AIR quotient calculation.
pub type DomainEvaluator<'a, SC> =
    dyn Fn(Domain<SC>) -> Result<Vec<Challenge<SC>>, PcsProverError<SC>> + 'a;

/// Trusted prover context; selectors always refer to the original trace domain.
/// `layout` includes main, preprocessing and periodic column counts.
/// Every declared periodic column's period divides the original trace height.
pub struct QuotientJob<'a, SC: SGC> {
    pub pcs: &'a SC::Pcs,
    pub instance_index: usize,
    pub trace_domain: Domain<SC>,
    pub quotient_domain: Domain<SC>,
    pub layout: AirLayout,
    pub main: CommittedMatrix<'a, SC>,
    pub permutation: Option<CommittedMatrix<'a, SC>>,
    pub preprocessed: Option<CommittedMatrix<'a, SC>>,
}

/// Return every full-domain quotient value in its original natural row order.
/// Implementations must validate allocation/geometry budgets before calling the evaluator.
pub trait QuotientScheduler<SC: SGC>: Sync {
    /// Default preserves the original batch parallelism; bounded schedulers may opt out.
    fn parallel_instances(&self) -> bool {
        true
    }

    fn evaluate(
        &self,
        job: &QuotientJob<'_, SC>,
        evaluate_domain: &DomainEvaluator<'_, SC>,
    ) -> Result<Vec<Challenge<SC>>, PcsProverError<SC>>;
}

/// Default scheduling: the original single full-domain evaluator invocation.
pub struct WholeDomainQuotient;

impl<SC: SGC> QuotientScheduler<SC> for WholeDomainQuotient {
    fn evaluate(
        &self,
        job: &QuotientJob<'_, SC>,
        evaluate_domain: &DomainEvaluator<'_, SC>,
    ) -> Result<Vec<Challenge<SC>>, PcsProverError<SC>> {
        evaluate_domain(job.quotient_domain)
    }
}
