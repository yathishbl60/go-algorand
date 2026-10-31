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

func Key(asset uint64) (string, error) {
	if asset == 0 {
		return "", ErrRecord
	}
	return string(binary.BigEndian.AppendUint64([]byte("aprv:z:"), asset)), nil
}

func Encode(record Record) ([]byte, error) {
	if err := record.Validate(); err != nil {
		return nil, err
	}
	encoded := make([]byte, EncodedSize)
	encoded[0] = 1
	copy(encoded[1:33], record.Manager[:])
	copy(encoded[33:65], record.Creator[:])
	copy(encoded[65:129], record.Policy[:])
	copy(encoded[129:193], record.Suite[:])
	copy(encoded[193:257], record.Group[:])
	copy(encoded[257:289], record.Destroy[:])
	binary.BigEndian.PutUint64(encoded[289:297], record.Total)
	binary.BigEndian.PutUint64(encoded[297:305], record.Created)
	binary.BigEndian.PutUint64(encoded[305:313], record.Epoch)
	return encoded, nil
}
