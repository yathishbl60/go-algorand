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

func checkHeader(encoded []byte, plan *Plan, limits Limits) error {
	if err := plan.validate(); err != nil {
		return err
	}
	if err := limits.validate(); err != nil {
		return err
	}
	if len(encoded) < HeaderSize || uint64(len(encoded)) > uint64(limits.MaxBundleBytes) {
		return ErrFrame
	}
	if string(encoded[:4]) != magic || encoded[4] != version ||
		int(binary.BigEndian.Uint16(encoded[5:7])) != len(plan.slots) {
		return ErrFrame
	}
	return nil
}

func (s Slot) matches(header []byte) bool {
	return Kind(header[0]) == s.kind && binary.BigEndian.Uint32(header[1:5]) == s.output && header[5] == s.role
}

func (s Slot) write(header []byte, length int) {
	header[0] = byte(s.kind)
	binary.BigEndian.PutUint32(header[1:5], s.output)
	header[5] = s.role
	binary.BigEndian.PutUint32(header[6:10], uint32(length))
}
