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

package keys

import "encoding/binary"

func Encode(record Record) ([]byte, error) {
	if err := record.Validate(); err != nil {
		return nil, err
	}
	b := append([]byte{1}, record.Domain.bytes()...)
	b = binary.BigEndian.AppendUint64(b, record.Epoch)
	b = binary.BigEndian.AppendUint64(b, record.Created)
	return append(b, record.PublicKey...), nil
}

func Decode(b []byte) (Record, error) {
	if len(b) < 154 || b[0] != 1 {
		return Record{}, ErrRecord
	}
	domain, err := decodeDomain(b[1:138])
	if err != nil {
		return Record{}, err
	}
	if len(b) != 154+domain.Role.publicKeySize() {
		return Record{}, ErrRecord
	}
	record := Record{domain, binary.BigEndian.Uint64(b[138:146]), binary.BigEndian.Uint64(b[146:154]), append([]byte(nil), b[154:]...)}
	return record, record.Validate()
}

func (d Domain) prefix() string     { return "aprv:k:" + string(d.bytes()) }
func (d Domain) currentKey() string { return d.prefix() + "c" }
func (d Domain) epochKey(epoch uint64) string {
	return d.prefix() + "e" + string(binary.BigEndian.AppendUint64(nil, epoch))
}
