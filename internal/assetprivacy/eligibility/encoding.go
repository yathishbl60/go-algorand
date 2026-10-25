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

func (r Record) Validate() error {
	if err := r.Domain.Validate(); err != nil {
		return err
	}
	if r.Revision == 0 {
		return ErrRevision
	}
	return nil
}

// Encode returns the exact 154-byte version1 effective-freeze snapshot.
func Encode(record Record) ([]byte, error) {
	if err := record.Validate(); err != nil {
		return nil, err
	}
	encoded := append([]byte{1}, record.Domain.bytes()...)
	encoded = binary.BigEndian.AppendUint64(encoded, record.Revision)
	encoded = binary.BigEndian.AppendUint64(encoded, record.Updated)
	frozen := byte(0)
	if record.Frozen {
		frozen = 1
	}
	return append(encoded, frozen), nil
}

func Decode(encoded []byte) (Record, error) {
	if len(encoded) != 154 || encoded[0] != 1 || encoded[153] > 1 {
		return Record{}, ErrRecord
	}
	domain, err := decodeDomain(encoded[1:137])
	if err != nil {
		return Record{}, err
	}
	record := Record{Domain: domain, Revision: binary.BigEndian.Uint64(encoded[137:145]),
		Updated: binary.BigEndian.Uint64(encoded[145:153]), Frozen: encoded[153] == 1}
	return record, record.Validate()
}
