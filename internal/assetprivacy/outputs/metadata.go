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

func (s *Store) metadata(count, round uint64) tree.Record {
	out := appendDomain([]byte{1}, s.domain)
	out = binary.BigEndian.AppendUint64(out, count)
	out = binary.BigEndian.AppendUint64(out, round)
	return tree.Record{Key: s.metadataKey(), Value: out}
}

func (s *Store) decodeMetadata(encoded []byte) error {
	if len(encoded) != 121 || encoded[0] != 1 {
		return ErrEncoding
	}
	if readDomain(encoded[1:105]) != s.domain {
		return ErrDomain
	}
	s.count = binary.BigEndian.Uint64(encoded[105:113])
	s.round = binary.BigEndian.Uint64(encoded[113:121])
	if s.count == 0 || s.count > tree.Capacity {
		return ErrState
	}
	return s.checkTail()
}

func (s *Store) checkTail() error {
	last, err := s.Read(uint32(s.count - 1))
	if err != nil {
		return err
	}
	if last.Round != s.round {
		return ErrState
	}
	return nil
}
