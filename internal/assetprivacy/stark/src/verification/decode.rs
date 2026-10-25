// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DecodeLimits, Error, Proof, reader::Reader, shape::Shape};
use p3_uni_stark::Commitments;

pub(super) const MAGIC: [u8; 8] = *b"NAPSTK\x01\0";

pub(super) fn proof(
    bytes: &[u8],
    limits: DecodeLimits,
    artifact: [u8; 64],
    degree: usize,
    shape: Shape,
) -> Result<Proof, Error> {
    let mut reader = Reader::new(bytes, limits)?;
    if reader.array::<8>()? != MAGIC || reader.array::<64>()? != artifact {
        return Err(Error::Artifact);
    }
    if usize::from(reader.byte()?) != degree {
        return Err(Error::Shape);
    }
    let proof = body(&mut reader, degree, shape)?;
    reader.finish()?;
    shape.check(&proof)?;
    Ok(proof)
}

fn body(reader: &mut Reader<'_>, degree_bits: usize, shape: Shape) -> Result<Proof, Error> {
    let commitments = Commitments {
        trace: reader.cap()?,
        quotient_chunks: reader.cap()?,
        random: Some(reader.cap()?),
    };
    Ok(Proof {
        commitments,
        degree_bits,
        opened_values: reader.openings(shape)?,
        opening_proof: (reader.random_openings()?, reader.fri()?),
        ood_pow_witness: reader.zero()?,
    })
}
