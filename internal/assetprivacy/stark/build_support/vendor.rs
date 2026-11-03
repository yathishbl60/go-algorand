// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub fn inputs() -> impl Iterator<Item = &'static str> {
    matrix().into_iter().chain(fri()).chain(batch())
}

fn matrix() -> [&'static str; 9] {
    [
        "vendor/p3-matrix/Cargo.toml",
        "vendor/p3-matrix/Cargo.upstream.toml",
        "vendor/p3-matrix/UPSTREAM.md",
        "vendor/p3-matrix/SHA3_512_MANIFEST.json",
        "vendor/p3-matrix/d10.patch",
        "vendor/p3-matrix/README.md",
        "vendor/p3-matrix/CHANGELOG.md",
        "vendor/p3-matrix/LICENSE-MIT",
        "vendor/p3-matrix/LICENSE-APACHE",
    ]
}

fn fri() -> [&'static str; 9] {
    [
        "vendor/p3-fri/Cargo.toml",
        "vendor/p3-fri/Cargo.upstream.toml",
        "vendor/p3-fri/UPSTREAM.md",
        "vendor/p3-fri/SHA3_512_MANIFEST.json",
        "vendor/p3-fri/input-matrix.patch",
        "vendor/p3-fri/README.md",
        "vendor/p3-fri/CHANGELOG.md",
        "vendor/p3-fri/LICENSE-MIT",
        "vendor/p3-fri/LICENSE-APACHE",
    ]
}

fn batch() -> [&'static str; 9] {
    [
        "vendor/p3-batch-stark/Cargo.toml",
        "vendor/p3-batch-stark/Cargo.upstream.toml",
        "vendor/p3-batch-stark/UPSTREAM.md",
        "vendor/p3-batch-stark/SHA3_512_MANIFEST.json",
        "vendor/p3-batch-stark/quotient-scheduler.patch",
        "vendor/p3-batch-stark/README.md",
        "vendor/p3-batch-stark/CHANGELOG.md",
        "vendor/p3-batch-stark/LICENSE-MIT",
        "vendor/p3-batch-stark/LICENSE-APACHE",
    ]
}
