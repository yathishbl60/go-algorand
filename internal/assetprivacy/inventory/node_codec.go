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

package inventory

import "encoding/binary"

func encodeNode(value node) []byte {
	encoded := make([]byte, nodeHeader+len(value.Key))
	encoded[0] = 1
	binary.BigEndian.PutUint16(encoded[1:3], uint16(len(value.Key)))
	binary.BigEndian.PutUint64(encoded[3:11], value.ValueBytes)
	encodeLink(encoded[11:76], value.Prev)
	encodeLink(encoded[76:141], value.Next)
	copy(encoded[nodeHeader:], value.Key)
	return encoded
}

func decodeNode(encoded []byte) (node, error) {
	if len(encoded) < nodeHeader || encoded[0] != 1 {
		return node{}, ErrIndex
	}
	size := int(binary.BigEndian.Uint16(encoded[1:3]))
	if size == 0 || size > MaxKeyBytes || len(encoded) != nodeHeader+size {
		return node{}, ErrIndex
	}
	prev, err := decodeLink(encoded[11:76])
	if err != nil {
		return node{}, err
	}
	next, err := decodeLink(encoded[76:141])
	return node{Key: string(encoded[nodeHeader:]), ValueBytes: binary.BigEndian.Uint64(encoded[3:11]), Prev: prev, Next: next}, err
}

func encodeLink(encoded []byte, value link) {
	if value.Exists {
		encoded[0] = 1
	}
	copy(encoded[1:], value.ID[:])
}

func decodeLink(encoded []byte) (link, error) {
	value := link{Exists: encoded[0] == 1}
	copy(value.ID[:], encoded[1:])
	if encoded[0] > 1 || (!value.Exists && value.ID != (ID{})) {
		return link{}, ErrIndex
	}
	return value, nil
}
