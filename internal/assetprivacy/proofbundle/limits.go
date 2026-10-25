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

package proofbundle

// Limits is trusted local admission configuration, not witness-advertised pricing.
// Allocation bounds owned codec buffers, excluding the caller's input and plans.
type Limits struct {
	MaxBundleBytes     uint32
	MaxProofBytes      uint32
	MaxAllocationBytes uint32
}

func (l Limits) validate() error {
	if l.MaxBundleBytes < HeaderSize || l.MaxBundleBytes > 512<<20 ||
		l.MaxProofBytes < starkHeader || l.MaxProofBytes > 128<<20 || l.MaxProofBytes > l.MaxBundleBytes ||
		l.MaxAllocationBytes == 0 || l.MaxAllocationBytes > 512<<20 {
		return ErrLimits
	}
	return nil
}

func (l Limits) checkProof(proof []byte) error {
	if len(proof) < starkHeader || uint64(len(proof)) > uint64(l.MaxProofBytes) || string(proof[:8]) != starkMagic {
		return ErrFrame
	}
	return nil
}
