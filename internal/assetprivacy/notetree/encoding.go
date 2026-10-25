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

package notetree

import (
	"bytes"
	"encoding/binary"

	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

func marker(domain tree.Domain, root tree.Hash, count uint64) []byte {
	encoded := append([]byte{2}, domainBytes(domain)...)
	encoded = binary.BigEndian.AppendUint64(encoded, count)
	return append(encoded, root[:]...)
}

func checkMarker(encoded []byte, domain tree.Domain, current *tree.Tree) error {
	if !bytes.Equal(encoded, marker(domain, current.Root(), current.Count())) {
		return ErrGeneration
	}
	return nil
}

func indexValue(domain tree.Domain, commitment tree.Hash, index uint32) []byte {
	encoded := append([]byte{2}, domainBytes(domain)...)
	encoded = append(encoded, commitment[:]...)
	return binary.BigEndian.AppendUint32(encoded, index)
}

func decodeIndex(encoded []byte, domain tree.Domain, commitment tree.Hash) (uint32, error) {
	if len(encoded) != 173 || encoded[0] != 2 {
		return 0, ErrEncoding
	}
	if !bytes.Equal(encoded[1:105], domainBytes(domain)) || !bytes.Equal(encoded[105:169], commitment[:]) {
		return 0, ErrEncoding
	}
	return binary.BigEndian.Uint32(encoded[169:]), nil
}
