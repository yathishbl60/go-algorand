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

package retirement

import "encoding/binary"

func Key(asset uint64) (string, error) {
	if asset == 0 {
		return "", ErrRecord
	}
	return string(binary.BigEndian.AppendUint64([]byte("aprv:x:"), asset)), nil
}

func Encode(record Record) ([]byte, error) {
	if err := record.Validate(); err != nil {
		return nil, err
	}
	encoded := make([]byte, EncodedSize)
	encoded[0] = 1
	copy(encoded[1:33], record.Manager[:])
	copy(encoded[33:97], record.Policy[:])
	copy(encoded[97:161], record.Suite[:])
	binary.BigEndian.PutUint64(encoded[161:169], record.Created)
	binary.BigEndian.PutUint64(encoded[169:177], record.Epoch)
	return encoded, nil
}

func Decode(encoded []byte) (Record, error) {
	if len(encoded) != EncodedSize || encoded[0] != 1 {
		return Record{}, ErrRecord
	}
	record := Record{Created: binary.BigEndian.Uint64(encoded[161:169]), Epoch: binary.BigEndian.Uint64(encoded[169:177])}
	copy(record.Manager[:], encoded[1:33])
	copy(record.Policy[:], encoded[33:97])
	copy(record.Suite[:], encoded[97:161])
	return record, record.Validate()
}
