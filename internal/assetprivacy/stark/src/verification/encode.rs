// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DecodeLimits, Error, Proof, decode::MAGIC, shape::Shape, writer::Writer};

pub(super) fn proof(
    proof: &Proof,
    limits: DecodeLimits,
    artifact: [u8; 64],
    degree: usize,
    shape: Shape,
) -> Result<Vec<u8>, Error> {
    shape.check(proof)?;
    if proof.degree_bits != degree {
        return Err(Error::Shape);
    }
    let mut writer = Writer::new(limits);
    writer.push(&MAGIC)?;
    writer.push(&artifact)?;
    writer.push(&[u8::try_from(degree).map_err(|_| Error::Shape)?])?;
    body(&mut writer, proof)?;
    Ok(writer.finish())
}

fn body(writer: &mut Writer, proof: &Proof) -> Result<(), Error> {
    writer.cap(&proof.commitments.trace)?;
    writer.cap(&proof.commitments.quotient_chunks)?;
    writer.cap(proof.commitments.random.as_ref().ok_or(Error::Shape)?)?;
    writer.openings(&proof.opened_values)?;
    writer.random_openings(&proof.opening_proof.0)?;
    writer.fri(&proof.opening_proof.1)?;
    writer.zero(&proof.ood_pow_witness)
}
