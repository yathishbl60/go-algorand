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

// Package outputs persists public encrypted notes for future wallet scanning.
// It does not verify proofs, identify recipients or authorize native operations.
package outputs

import (
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

// Record contains public data only. Key epochs and recipient identities stay private.
type Record struct {
	Domain      tree.Domain
	Index       uint32
	Round       uint64
	Transaction [32]byte
	Nonce       [32]byte
	Output      uint32
	Commitment  tree.Hash
	Disclosures policy.Disclosures
	Recipient   []byte
	Auditor     []byte
	Controller  []byte
}

// Store is a snapshot. Callers must serialize KV operations and reopen after writes.
// The KV must atomically apply batches or roll them back, and copy write buffers.
type Store struct {
	kv          tree.KV
	domain      tree.Domain
	disclosures policy.Disclosures
	count       uint64
	round       uint64
}

func (s *Store) Count() uint64     { return s.count }
func (s *Store) LastRound() uint64 { return s.round }

// Page is bounded to MaxPageSize records; Next can represent the full 2^32 count.
type Page struct {
	Records []Record
	Next    uint64
	Count   uint64
}

// MaxPageSize bounds this component's query allocations, not a consensus parameter.
const MaxPageSize uint32 = 64
const headerSize = 250
