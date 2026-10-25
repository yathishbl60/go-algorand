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

package disclosure

import "encoding/binary"

// Encode returns the exact 186-byte versioned current-assignment record.
func Encode(record Record) ([]byte, error) {
	if err := record.Validate(); err != nil {
		return nil, err
	}
	encoded := append([]byte{1}, record.Domain.bytes()...)
	encoded = append(encoded, byte(record.Flags))
	encoded = append(encoded, record.Auditor[:]...)
	encoded = append(encoded, record.Controller[:]...)
	encoded = binary.BigEndian.AppendUint64(encoded, record.Epoch)
	return binary.BigEndian.AppendUint64(encoded, record.Updated), nil
}

func Decode(encoded []byte) (Record, error) {
	if len(encoded) != 186 || encoded[0] != 1 {
		return Record{}, ErrRecord
	}
	domain, err := decodeDomain(encoded[1:105])
	if err != nil {
		return Record{}, err
	}
	record := Record{Domain: domain, Flags: Flags(encoded[105])}
	copy(record.Auditor[:], encoded[106:138])
	copy(record.Controller[:], encoded[138:170])
	record.Epoch = binary.BigEndian.Uint64(encoded[170:178])
	record.Updated = binary.BigEndian.Uint64(encoded[178:186])
	return record, record.Validate()
}
