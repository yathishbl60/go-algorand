// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::{DecodeLimits, Error, writer::Writer};
use super::{BundleLimits, ENTRY, Expected, HEADER, MAGIC};

/// Frames existing per-AIR proofs in trusted expected order; it does not verify them.
pub fn encode_bundle(
    slots: &[Expected<'_>],
    proofs: &[&[u8]],
    limits: BundleLimits,
) -> Result<Vec<u8>, Error> {
    size(slots, proofs, limits)?;
    let mut writer = Writer::bounded(limits.wire.min(limits.allocation));
    writer.push(&MAGIC)?;
    writer.push(&(slots.len() as u16).to_be_bytes())?;
    for (slot, proof) in slots.iter().zip(proofs) {
        entry(&mut writer, slot, proof)?;
    }
    Ok(writer.finish())
}

fn size(slots: &[Expected<'_>], proofs: &[&[u8]], limits: BundleLimits) -> Result<(), Error> {
    super::layout::check(slots)?;
    if proofs.len() != slots.len() {
        return Err(Error::Shape);
    }
    let bytes = proof_bytes(slots, proofs, limits)?;
    if bytes > limits.wire {
        return Err(Error::WireLimit);
    }
    if bytes > limits.allocation {
        return Err(Error::MemoryLimit);
    }
    Ok(())
}

fn proof_bytes(
    slots: &[Expected<'_>],
    proofs: &[&[u8]],
    limits: BundleLimits,
) -> Result<usize, Error> {
    let proof_limits = DecodeLimits::new(limits.proof, limits.allocation)?;
    let mut bytes = HEADER + ENTRY * slots.len();
    for (slot, proof) in slots.iter().zip(proofs) {
        limits.proof(proof)?;
        slot.target.header(proof, proof_limits)?;
        bytes = bytes.checked_add(proof.len()).ok_or(Error::WireLimit)?;
    }
    Ok(bytes)
}

fn entry(writer: &mut Writer, slot: &Expected<'_>, proof: &[u8]) -> Result<(), Error> {
    writer.push(&[slot.kind])?;
    writer.push(&slot.output.to_be_bytes())?;
    writer.push(&[slot.role])?;
    writer.push(&(proof.len() as u32).to_be_bytes())?;
    writer.push(proof)
}
