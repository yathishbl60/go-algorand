// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn collect(root: &Path) -> io::Result<Vec<String>> {
    let mut names = fixed_inputs();
    let mut directories = vec![
        root.join("stark/src"),
        root.join("stark/build_support"),
        root.join("stark-ffi/src"),
    ];
    while let Some(directory) = directories.pop() {
        for entry in fs::read_dir(directory)? {
            add(root, entry?, &mut directories, &mut names)?;
        }
    }
    names.sort();
    Ok(names)
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
        "stark/build.rs",
        "stark-ffi/Cargo.toml",
        "stark-ffi/include/nap_privacy.h",
    ]
    .map(String::from)
    .to_vec()
}
