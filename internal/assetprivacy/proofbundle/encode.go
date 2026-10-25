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

import "encoding/binary"

// Encode always emits the trusted plan's order/kinds, never caller descriptors.
// It serializes opaque per-AIR proofs without verifying them.
func Encode(plan *Plan, proofs [][]byte, limits Limits) ([]byte, error) {
	if err := limits.validate(); err != nil {
		return nil, err
	}
	size, err := encodedSize(plan, proofs, limits)
	if err != nil {
		return nil, err
	}
	encoded := make([]byte, size)
	copy(encoded, magic)
	encoded[4] = version
	binary.BigEndian.PutUint16(encoded[5:7], uint16(len(plan.slots)))
	offset := HeaderSize
	for index, slot := range plan.slots {
		slot.write(encoded[offset:offset+EntrySize], len(proofs[index]))
		offset += EntrySize
		offset += copy(encoded[offset:], proofs[index])
	}
	return encoded, nil
}
