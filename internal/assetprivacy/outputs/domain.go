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

package outputs

import (
	"encoding/binary"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

func checkDomain(d tree.Domain) error {
	if d.Asset == 0 || d.Genesis == [32]byte{} || d.Policy == [64]byte{} {
		return ErrDomain
	}
	return nil
}

func appendDomain(out []byte, d tree.Domain) []byte {
	out = append(out, d.Genesis[:]...)
	out = binary.BigEndian.AppendUint64(out, d.Asset)
	return append(out, d.Policy[:]...)
}

// readDomain is called only after the enclosing fixed header length is checked.
func readDomain(encoded []byte) tree.Domain {
	d := tree.Domain{Asset: binary.BigEndian.Uint64(encoded[32:40])}
	copy(d.Genesis[:], encoded[:32])
	copy(d.Policy[:], encoded[40:104])
	return d
}

func (s *Store) prefix() string {
	return "aprv:o:" + string(binary.BigEndian.AppendUint64(nil, s.domain.Asset))
}

func (s *Store) metadataKey() string { return s.prefix() + "m" }
func (s *Store) recordKey(index uint32) string {
	return s.prefix() + "e" + string(binary.BigEndian.AppendUint32(nil, index))
}
