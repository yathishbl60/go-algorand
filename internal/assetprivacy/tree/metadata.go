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

package tree

import (
	"bytes"
	"encoding/binary"
)

func (t *Tree) metadata(root Hash, count uint64) Record {
	encoded := append([]byte{1}, t.domain.Genesis[:]...)
	encoded = append(encoded, t.domain.Policy[:]...)
	encoded = binary.BigEndian.AppendUint64(encoded, count)
	encoded = append(encoded, root[:]...)
	return Record{t.domain.metadataKey(), encoded}
}

func (t *Tree) decodeMetadata(encoded []byte) error {
	if len(encoded) != 169 || encoded[0] != 1 {
		return ErrEncoding
	}
	if !bytes.Equal(encoded[1:33], t.domain.Genesis[:]) {
		return ErrDomain
	}
	if !bytes.Equal(encoded[33:97], t.domain.Policy[:]) {
		return ErrDomain
	}
	t.count = binary.BigEndian.Uint64(encoded[97:105])
	copy(t.root[:], encoded[105:])
	if t.count > Capacity {
		return ErrEncoding
	}
	if (t.count == 0) != (t.root == t.empty[Depth]) {
		return ErrEncoding
	}
	return nil
}
