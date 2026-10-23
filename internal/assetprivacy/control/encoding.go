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

package control

import "encoding/binary"

func (r Record) Validate() error {
	if err := r.Domain.Validate(); err != nil {
		return err
	}
	if r.Epoch == 0 {
		return ErrEpoch
	}
	return nil
}
func Encode(record Record) ([]byte, error) {
	if err := record.Validate(); err != nil {
		return nil, err
	}
	b := append([]byte{1}, record.Domain.bytes()...)
	b = binary.BigEndian.AppendUint64(b, record.Epoch)
	b = binary.BigEndian.AppendUint64(b, record.Updated)
	frozen := byte(0)
	if record.Frozen {
		frozen = 1
	}
	return append(b, frozen), nil
}
func Decode(b []byte) (Record, error) {
	if len(b) != 154 || b[0] != 1 || b[153] > 1 {
		return Record{}, ErrRecord
	}
	domain, err := decodeDomain(b[1:137])
	if err != nil {
		return Record{}, err
	}
	record := Record{domain, binary.BigEndian.Uint64(b[137:145]), binary.BigEndian.Uint64(b[145:153]), b[153] == 1}
	return record, record.Validate()
}
