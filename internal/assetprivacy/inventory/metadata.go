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

func encodeMetadata(record Record) []byte {
	encoded := make([]byte, MetadataSize)
	encoded[0] = 1
	binary.BigEndian.PutUint64(encoded[1:9], record.Items)
	binary.BigEndian.PutUint64(encoded[9:17], record.Bytes)
	copy(encoded[17:81], record.Head[:])
	copy(encoded[81:145], record.Tail[:])
	return encoded
}

func decodeMetadata(encoded []byte) (Record, error) {
	if len(encoded) != MetadataSize || encoded[0] != 1 {
		return Record{}, ErrIndex
	}
	record := Record{Items: binary.BigEndian.Uint64(encoded[1:9]), Bytes: binary.BigEndian.Uint64(encoded[9:17])}
	copy(record.Head[:], encoded[17:81])
	copy(record.Tail[:], encoded[81:145])
	return record, record.Validate()
}

func Read(kv Getter, asset uint64) (Record, bool, error) {
	key, err := MetadataKey(asset)
	if err != nil || kv == nil {
		return Record{}, false, ErrIndex
	}
	encoded, exists, err := kv.Get(key)
	if err != nil || !exists {
		return Record{}, exists, err
	}
	record, err := decodeMetadata(encoded)
	return record, true, err
}
