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

package proofbridge

import "github.com/algorand/go-algorand/internal/assetprivacy/proofbundle"

// Limits are trusted local caps, not proof-advertised work or consensus pricing.
// Go's owned payload copy and Rust's decoder allocation have separate equal caps.
type Limits struct {
	MaxBundleBytes     uint32
	MaxProofBytes      uint32
	MaxAllocationBytes uint32
}

// Validate checks the shared deterministic guard ceilings; it grants no suite approval.
func (l Limits) Validate() error { return l.validate() }

func (l Limits) validate() error {
	if l.MaxBundleBytes < proofbundle.HeaderSize || l.MaxBundleBytes > 512<<20 ||
		l.MaxProofBytes < 73 || l.MaxProofBytes > 128<<20 || l.MaxProofBytes > l.MaxBundleBytes ||
		l.MaxAllocationBytes == 0 || l.MaxAllocationBytes > 512<<20 {
		return ErrLimits
	}
	return nil
}

func (l Limits) bundle() proofbundle.Limits {
	return proofbundle.Limits{MaxBundleBytes: l.MaxBundleBytes, MaxProofBytes: l.MaxProofBytes,
		MaxAllocationBytes: l.MaxAllocationBytes}
}
