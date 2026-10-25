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

// Package eligibility persists authoritative effective-freeze snapshots.
// It does not authenticate account consent, issuer authority or proofs.
package eligibility

import (
	"errors"

	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

const namespace = "aprv:eligibility:"

var (
	ErrDomain   = errors.New("invalid privacy eligibility domain")
	ErrRecord   = errors.New("invalid privacy eligibility record")
	ErrHistory  = errors.New("inconsistent privacy eligibility current state")
	ErrRevision = errors.New("invalid privacy eligibility revision")
	ErrRound    = errors.New("privacy eligibility update is from a future round")
)

type Domain struct {
	Network [32]byte
	Asset   uint64
	Policy  [64]byte
	Account [32]byte
}

// Record is a derived current effective-freeze snapshot, not a raw control marker.
// Revision is independent of both control-marker epochs and key epochs.
type Record struct {
	Domain   Domain
	Revision uint64
	Updated  uint64
	Frozen   bool
}

// Root cannot be passed directly as a historical note or current-key root.
type Root struct{ Digest tree.Hash }

// Store requires serialized KV access and atomic Apply with rollback on failure.
// It retains no advanced Tree snapshot across operations or failed writes.
type Store struct {
	kv     tree.KV
	domain tree.Domain
}

type indexedRecord struct {
	index  uint32
	record Record
}
