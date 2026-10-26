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

package artifactregistry

import "encoding/binary"

func decodeEntry(encoded []byte, expected layout) (Entry, error) {
	entry := Entry{
		Kind:        Kind(encoded[0]),
		Rows:        binary.LittleEndian.Uint32(encoded[1:5]),
		MainWidth:   binary.LittleEndian.Uint32(encoded[5:9]),
		PreWidth:    binary.LittleEndian.Uint32(encoded[9:13]),
		PublicCount: binary.LittleEndian.Uint32(encoded[13:17]),
		Flags:       encoded[17],
	}
	if !entry.matches(expected) {
		return Entry{}, ErrLayout
	}
	for index := range entry.Root {
		entry.Root[index] = binary.LittleEndian.Uint64(encoded[18+index*8 : 26+index*8])
	}
	return entry, nil
}
