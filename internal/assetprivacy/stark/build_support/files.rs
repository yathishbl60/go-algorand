// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::vendor;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn collect(root: &Path) -> io::Result<Vec<String>> {
    let mut names = fixed_inputs();
    let mut directories = source_directories().map(|path| root.join(path)).to_vec();
    while let Some(directory) = directories.pop() {
        for entry in fs::read_dir(directory)? {
            add(root, entry?, &mut directories, &mut names)?;
        }
    }
    names.sort();
    Ok(names)
}

pub fn source_directories() -> [&'static str; 7] {
    [
        "stark/src",
        "stark/build_support",
        "stark-ffi/src",
        "field31/src",
        "matrix-adapter/src",
        "vendor/p3-matrix/src",
        "vendor/p3-fri/src",
    ]
}

fn add(
    root: &Path,
    entry: fs::DirEntry,
    dirs: &mut Vec<PathBuf>,
    names: &mut Vec<String>,
) -> io::Result<()> {
    let kind = entry.file_type()?;
    let path = entry.path();
    if kind.is_symlink() {
        return Err(io::ErrorKind::InvalidData.into());
    }
    if kind.is_dir() {
        dirs.push(path);
    } else if kind.is_file() && path.extension().is_some_and(|value| value == "rs") {
        names.push(relative(root, &path)?);
    }
    Ok(())
}

fn relative(root: &Path, path: &Path) -> io::Result<String> {
    let relative = path
        .strip_prefix(root)
        .map_err(|_| io::ErrorKind::InvalidInput)?;
    let parts: Option<Vec<_>> = relative
        .components()
        .map(|part| part.as_os_str().to_str())
        .collect();
    Ok(parts.ok_or(io::ErrorKind::InvalidData)?.join("/"))
}

fn fixed_inputs() -> Vec<String> {
    [
        "Cargo.toml",
        "Cargo.lock",
        "rust-toolchain.toml",
        "stark/Cargo.toml",
        "field31/Cargo.toml",
        "stark/build.rs",
        "stark-ffi/Cargo.toml",
        "stark-ffi/include/nap_privacy.h",
        "stark/patches/p3-fri-input-matrix.patch",
    ]
    .into_iter()
    .chain(vendor::inputs())
    .chain(adapter_inputs())
    .map(String::from)
    .collect()
}

fn adapter_inputs() -> [&'static str; 2] {
    ["matrix-adapter/Cargo.toml", "matrix-adapter/README.md"]
}
