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

package note

import "encoding/binary"

// Decode rejects alternate versions, lengths, fields and noncanonical padding.
func Decode(encoded []byte) (Note, error) {
	if !canonical(encoded) {
		return Note{}, ErrNote
	}
	n := Note{Asset: binary.BigEndian.Uint64(encoded[1:9])}
	copy(n.Policy[:], encoded[9:73])
	copy(n.Owner[:], encoded[73:137])
	copy(n.Account[:], encoded[137:169])
	n.Value = binary.BigEndian.Uint64(encoded[169:177])
	copy(n.Serial[:], encoded[177:209])
	copy(n.Blinding[:], encoded[209:241])
	if err := n.Validate(); err != nil {
		return Note{}, err
	}
	return n, nil
}

func canonical(encoded []byte) bool {
	if len(encoded) != EncodedSize || encoded[0] != version {
		return false
	}
	for _, value := range encoded[contentSize:] {
		if value != 0 {
			return false
		}
	}
	return true
}
