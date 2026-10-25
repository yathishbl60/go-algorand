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

// Decode returns independent owned proof bytes after full structural preflight.
// Successful decoding is never proof verification or native authorization.
// Charge requested proof bytes and at most three 64-bit words per Go slice.
func Decode(encoded []byte, plan *Plan, limits Limits) ([][]byte, error) {
	if err := Check(encoded, plan, limits); err != nil {
		return nil, err
	}
	charge := uint64(len(encoded)) + uint64(len(plan.slots))*24
	if charge > uint64(limits.MaxAllocationBytes) {
		return nil, ErrLimits
	}
	proofs := make([][]byte, len(plan.slots))
	r := reader{remaining: encoded[HeaderSize:]}
	for index, slot := range plan.slots {
		proof, err := r.next(slot, limits)
		if err != nil {
			return nil, err
		}
		proofs[index] = make([]byte, len(proof))
		copy(proofs[index], proof)
	}
	return proofs, nil
}
