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

import (
	"encoding/binary"

	"github.com/algorand/go-algorand/internal/assetprivacy/proofbundle"
)

func writeSlot(encoded []byte, at int, slot proofbundle.Slot) int {
	encoded[at] = byte(slot.Kind())
	binary.BigEndian.PutUint32(encoded[at+1:at+5], slot.Output())
	encoded[at+5] = slot.Role()
	values := slot.Values()
	binary.BigEndian.PutUint16(encoded[at+6:at+8], uint16(len(values)))
	at += 8
	for _, value := range values {
		binary.LittleEndian.PutUint16(encoded[at:at+2], value)
		at += 2
	}
	return at
}
