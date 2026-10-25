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

package eligibility

import "encoding/binary"

func encodeIndex(indexed indexedRecord) ([]byte, error) {
	encoded, err := Encode(indexed.record)
	if err != nil {
		return nil, err
	}
	return append(binary.BigEndian.AppendUint32(nil, indexed.index), encoded...), nil
}

func (s *Store) readIndex(domain Domain, round uint64) (indexedRecord, bool, error) {
	encoded, exists, err := s.kv.Get(indexKey(domain))
	if err != nil || !exists {
		return indexedRecord{}, exists, err
	}
	if len(encoded) != 158 {
		return indexedRecord{}, true, ErrRecord
	}
	record, err := Decode(encoded[4:])
	if err != nil {
		return indexedRecord{}, true, err
	}
	if record.Domain != domain {
		return indexedRecord{}, true, ErrDomain
	}
	if record.Updated > round {
		return indexedRecord{}, true, ErrRound
	}
	return indexedRecord{index: binary.BigEndian.Uint32(encoded[:4]), record: record}, true, nil
}
