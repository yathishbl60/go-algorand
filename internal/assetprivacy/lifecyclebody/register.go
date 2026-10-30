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

	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
)

func (b Body) validateRegistration() error {
	if b.Policy == [64]byte{} || b.Account == [32]byte{} || b.ExpectedEpoch == ^uint64(0) {
		return ErrHeader
	}
	if b.Modes != 0 || b.Capabilities != 0 || b.Disclosures != 0 || b.Auditor != [32]byte{} || b.Target != [32]byte{} || b.Frozen {
		return ErrFields
	}
	if registrationKeySize(b.Role) == 0 {
		return ErrOperation
	}
	return keys.ValidatePublicKey(b.Role, b.PublicKey)
}

// The caller has already checked the complete operation's exact length.
func (b *Body) readRegistrationTail(encoded []byte) error {
	if int(binary.BigEndian.Uint16(encoded[33:35])) != registrationKeySize(keys.Role(encoded[0])) {
		return ErrEncoding
	}
	b.Role = keys.Role(encoded[0])
	copy(b.Account[:], encoded[1:33])
	b.PublicKey = encoded[35:]
	return nil
}

func writeRegistrationTail(encoded []byte, body Body) {
	encoded[0] = byte(body.Role)
	copy(encoded[1:33], body.Account[:])
	binary.BigEndian.PutUint16(encoded[33:35], uint16(len(body.PublicKey)))
	copy(encoded[35:], body.PublicKey)
}
