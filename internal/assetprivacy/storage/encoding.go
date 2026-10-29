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

package storage

import "encoding/binary"

func Key(payer [32]byte) (string, error) {
	if payer == [32]byte{} {
		return "", ErrPayer
	}
	return keyPrefix + string(payer[:]), nil
}

func Encode(o Obligation) ([]byte, error) {
	if err := o.Validate(); err != nil {
		return nil, err
	}
	encoded := binary.BigEndian.AppendUint64([]byte{1}, o.Items)
	return binary.BigEndian.AppendUint64(encoded, o.Bytes), nil
}

func Decode(encoded []byte) (Obligation, error) {
	if len(encoded) != EncodedSize || encoded[0] != 1 {
		return Obligation{}, ErrEncoding
	}
	o := Obligation{Items: binary.BigEndian.Uint64(encoded[1:9]), Bytes: binary.BigEndian.Uint64(encoded[9:])}
	return o, o.Validate()
}
