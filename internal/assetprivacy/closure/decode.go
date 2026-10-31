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

package closure

import "encoding/binary"

func Decode(encoded []byte) (Record, error) {
	if len(encoded) != EncodedSize || encoded[0] != 1 {
		return Record{}, ErrRecord
	}
	record := Record{Total: binary.BigEndian.Uint64(encoded[289:297]),
		Created: binary.BigEndian.Uint64(encoded[297:305]), Epoch: binary.BigEndian.Uint64(encoded[305:313])}
	copy(record.Manager[:], encoded[1:33])
	copy(record.Creator[:], encoded[33:65])
	copy(record.Policy[:], encoded[65:129])
	copy(record.Suite[:], encoded[129:193])
	copy(record.Group[:], encoded[193:257])
	copy(record.Destroy[:], encoded[257:289])
	return record, record.Validate()
}
