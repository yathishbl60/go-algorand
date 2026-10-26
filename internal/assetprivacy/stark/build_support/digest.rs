// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use sha3::{Digest, Sha3_512};
use std::{fs, io, path::Path};

pub fn source(root: &Path, names: &[String]) -> io::Result<[u8; 64]> {
    let mut hash = Sha3_512::new();
    hash.update(b"Algorand/AssetPrivacy/Candidate1/ImplementationSourceV1/");
    let count = u32::try_from(names.len()).map_err(|_| io::ErrorKind::InvalidData)?;
    hash.update(count.to_le_bytes());
    for name in names {
        let length = u32::try_from(name.len()).map_err(|_| io::ErrorKind::InvalidData)?;
        let bytes = fs::read(root.join(name))?;
        hash.update(length.to_le_bytes());
        hash.update(name.as_bytes());
        hash.update((bytes.len() as u64).to_le_bytes());
        hash.update(bytes);
    }
    Ok(hash.finalize().into())
}
