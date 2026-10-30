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

package lifecyclebody

// IsLifecycle recognizes the family only; Decode must still check its version.
func IsLifecycle(encoded []byte) bool {
	return len(encoded) >= 4 && string(encoded[:4]) == "NAPL"
}

// Decode validates exact lengths and material before copying a registration key.
// Returned fields retain no input buffer. Keep encoded immutable until return.
func Decode(encoded []byte) (Body, error) {
	if len(encoded) < HeaderSize || string(encoded[:5]) != "NAPL\x01" {
		return Body{}, ErrEncoding
	}
	size, err := encodedBodySize(encoded, Kind(encoded[5]))
	if err != nil {
		return Body{}, err
	}
	if len(encoded) != size {
		return Body{}, ErrEncoding
	}
	body := readHeader(encoded)
	return body.decodeTail(encoded[HeaderSize:])
}
