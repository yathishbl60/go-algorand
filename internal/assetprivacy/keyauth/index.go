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

package keyauth

import (
	"bytes"
	"encoding/binary"

	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
)

func encodeIndex(record indexRecord) []byte {
	encoded := append([]byte{1}, domainBytes(record.domain)...)
	encoded = binary.BigEndian.AppendUint32(encoded, record.index)
	encoded = append(encoded, record.leaf[:]...)
	return binary.BigEndian.AppendUint64(encoded, record.epoch)
}

func decodeIndex(encoded []byte, domain keys.Domain) (indexRecord, error) {
	if len(encoded) != indexSize || encoded[0] != 1 {
		return indexRecord{}, ErrRecord
	}
	if !bytes.Equal(encoded[1:138], domainBytes(domain)) {
		return indexRecord{}, ErrDomain
	}
	record := indexRecord{domain: domain, index: binary.BigEndian.Uint32(encoded[138:142])}
	copy(record.leaf[:], encoded[142:206])
	record.epoch = binary.BigEndian.Uint64(encoded[206:214])
	if record.epoch == 0 {
		return indexRecord{}, ErrRecord
	}
	return record, nil
}

func (s *Store) readIndex(domain keys.Domain) (indexRecord, bool, error) {
	encoded, exists, err := s.kv.Get(indexKey(domain))
	if err != nil || !exists {
		return indexRecord{}, exists, err
	}
	record, err := decodeIndex(encoded, domain)
	return record, true, err
}
