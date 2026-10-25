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

package proofplan

import "encoding/binary"

// envelopeValues writes the common 1100-scalar prefix for output role proofs.
// Build validates wire length/version and policy role presence before calling it.
func envelopeValues(context [248]byte, wire []byte, root [64]byte) []uint16 {
	values := make([]uint16, 0, DisclosurePublicValues)
	values = appendBytes(values, context[:])
	values = appendLimbs(values, wire[1:1089])
	values = appendBytes(values, wire[1089:1101])
	values = appendLimbs(values, wire[1101:1613])
	values = appendLimbs(values, wire[1613:1629])
	return appendLimbs(values, root[:])
}

func appendBytes(values []uint16, bytes []byte) []uint16 {
	for _, value := range bytes {
		values = append(values, uint16(value))
	}
	return values
}

func appendLimbs(values []uint16, bytes []byte) []uint16 {
	for offset := 0; offset < len(bytes); offset += 2 {
		values = append(values, binary.LittleEndian.Uint16(bytes[offset:offset+2]))
	}
	return values
}
