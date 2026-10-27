// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

mod digest;
mod files;

use std::{env, fs, io, path::PathBuf};

pub fn emit() -> io::Result<()> {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").ok_or(io::ErrorKind::NotFound)?);
    let output = PathBuf::from(env::var_os("OUT_DIR").ok_or(io::ErrorKind::NotFound)?);
    let root = root.parent().ok_or(io::ErrorKind::InvalidInput)?;
    let paths = files::collect(root)?;
    let identity = digest::source(root, &paths)?;
    fs::write(output.join("nap_source_id.rs"), format!("{identity:?}\n"))?;
    for path in paths {
        println!("cargo:rerun-if-changed={}", root.join(path).display());
    }
    for directory in ["stark/src", "stark/build_support", "stark-ffi/src"] {
        println!("cargo:rerun-if-changed={}", root.join(directory).display());
    }
    Ok(())
}
