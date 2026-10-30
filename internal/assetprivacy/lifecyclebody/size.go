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

import (
	"encoding/binary"

	"github.com/algorand/go-algorand/crypto"
	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
)

func registrationKeySize(role keys.Role) int {
	if role == keys.RecipientSpend {
		return 64
	}
	if role.UsesStarkPossession() {
		return 1184
	}
	if role.UsesFalconPossession() {
		return crypto.Falcon1024PublicKeySize
	}
	return 0
}

func registrationBodySize(role keys.Role) int {
	size := registrationKeySize(role)
	if size == 0 {
		return 0
	}
	return HeaderSize + 35 + size
}

// Check role and its exact declared length before any tail read or owned copy.
func encodedBodySize(encoded []byte, operation Kind) (int, error) {
	if operation != Register {
		size := operation.encodedSize(0)
		if size == 0 {
			return 0, ErrOperation
		}
		return size, nil
	}
	if len(encoded) < HeaderSize+35 {
		return 0, ErrEncoding
	}
	return encodedRegistrationSize(encoded[HeaderSize:])
}

func encodedRegistrationSize(tail []byte) (int, error) {
	size := registrationKeySize(keys.Role(tail[0]))
	if size == 0 {
		return 0, ErrOperation
	}
	if int(binary.BigEndian.Uint16(tail[33:35])) != size {
		return 0, ErrEncoding
	}
	return HeaderSize + 35 + size, nil
}
