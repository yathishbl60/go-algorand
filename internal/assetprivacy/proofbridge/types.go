//go:build algorand_native_privacy

// Copyright (C) 2019-2026 Algorand Foundation Ltd.
// This file is part of go-algorand
//
// go-algorand is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// go-algorand is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with go-algorand.  If not, see <https://www.gnu.org/licenses/>.

// Package proofbridge checks native expected bundles through an optional Rust backend.
// Cryptographic success is not complete native transaction authorization.
package proofbridge

import (
	"errors"

	"github.com/algorand/go-algorand/internal/assetprivacy/artifactregistry"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

var (
	ErrUnavailable = errors.New("native privacy Rust verifier is unavailable")
	ErrRejected    = errors.New("native privacy Rust verifier rejected the proof bundle")
	ErrContext     = errors.New("invalid native privacy verifier context")
	ErrLimits      = errors.New("invalid native privacy verifier limits")
)

// Context must be copied from an authoritative current native plan, never the wire.
type Context struct {
	Domain tree.Domain
	Suite  [64]byte
	Group  [64]byte
}

// Verifier owns the independently pinned registry and trusted immutable budgets.
// A zero value cannot verify, and no callback or fallback can report success.
type Verifier struct {
	registry *artifactregistry.Registry
	encoded  [artifactregistry.EncodedSize]byte
	pin      artifactregistry.Pin
	limits   Limits
}
