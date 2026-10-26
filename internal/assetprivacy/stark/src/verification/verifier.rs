// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Artifact, DecodeLimits, Error, Proof, decode, encode, metadata, shape::Shape};
use crate::{Config, Value};
use p3_air::{Air, BaseAir};
use p3_field::PrimeCharacteristicRing;
use p3_uni_stark::{SymbolicAirBuilder, VerifierConstraintFolder, verify_with_preprocessed};
use std::panic::{AssertUnwindSafe, catch_unwind};

/// Candidate cryptographic checking only; the native caller still authenticates all context.
pub struct Verifier<A> {
    air: A,
    pub(super) artifact: Artifact,
    pub(super) identity: [u8; 64],
    pub(super) shape: Shape,
}

impl<A: BaseAir<Value>> Verifier<A> {
    pub(super) fn from_trusted(air: A, artifact: Artifact) -> Result<Self, Error> {
        let shape = Shape::from_air(&air)?;
        if artifact.key.width != shape.preprocessed {
            return Err(Error::Artifact);
        }
        let identity = metadata::identity(&artifact, shape);
        Ok(Self {
            air,
            artifact,
            identity,
            shape,
        })
    }

    /// Bounded canonical serialization only; this does not verify the proof.
    pub fn encode(&self, proof: &Proof, limits: DecodeLimits) -> Result<Vec<u8>, Error> {
        encode::proof(
            proof,
            limits,
            self.identity,
            self.artifact.key.degree_bits,
            self.shape,
        )
    }

    fn decode(&self, bytes: &[u8], limits: DecodeLimits) -> Result<Proof, Error> {
        decode::proof(
            bytes,
            limits,
            self.identity,
            self.artifact.key.degree_bits,
            self.shape,
        )
    }
}

impl<A> Verifier<A>
where
    A: Air<SymbolicAirBuilder<Value>> + for<'a> Air<VerifierConstraintFolder<'a, Config>>,
{
    /// `public_values` must be the exact ledger-derived statement, never copied from the proof.
    pub fn verify(
        &self,
        bytes: &[u8],
        public_values: &[u16],
        limits: DecodeLimits,
    ) -> Result<(), Error> {
        if public_values.len() != self.shape.public {
            return Err(Error::PublicValues);
        }
        let proof = self.decode(bytes, limits)?;
        self.verify_decoded(&proof, public_values)
    }

    fn verify_decoded(&self, proof: &Proof, public_values: &[u16]) -> Result<(), Error> {
        let values: Vec<_> = public_values.iter().copied().map(Value::from_u16).collect();
        catch_unwind(AssertUnwindSafe(|| {
            verify_with_preprocessed(
                &crate::config::verifier_config(),
                &self.air,
                proof,
                &values,
                Some(&self.artifact.key),
            )
            .map_err(|_| Error::InvalidProof)
        }))
        .map_err(|_| Error::BackendPanic)?
    }
}
