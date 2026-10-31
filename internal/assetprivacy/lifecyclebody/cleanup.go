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

package lifecyclebody

import "encoding/binary"

const MaxCleanupRecords = 32

func (b Body) validateOperation() error {
	if b.Operation.encodedSize(b.Role) == 0 {
		return ErrOperation
	}
	if b.Operation != Cleanup && b.CleanupLimit != 0 {
		return ErrFields
	}
	return nil
}

func (b Body) validateCleanup() error {
	if b.Operation != Retire && b.Operation != Cleanup {
		return nil
	}
	if b.Auditor != [32]byte{} || b.Target != [32]byte{} || b.Frozen {
		return ErrFields
	}
	if b.Operation == Retire && b.ExpectedEpoch != 0 {
		return ErrFields
	}
	if b.Operation == Cleanup && (b.ExpectedEpoch == 0 || b.ExpectedEpoch == ^uint64(0) || b.CleanupLimit == 0 || b.CleanupLimit > MaxCleanupRecords) {
		return ErrFields
	}
	return nil
}

func (b *Body) readCleanupTail(encoded []byte) {
	b.CleanupLimit = binary.BigEndian.Uint16(encoded)
}

func writeCleanupTail(encoded []byte, body Body) {
	binary.BigEndian.PutUint16(encoded, body.CleanupLimit)
}
