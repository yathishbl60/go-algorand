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

import (
	"github.com/algorand/go-algorand/crypto"
	"github.com/algorand/go-algorand/internal/assetprivacy/envelope"
)

// ValidatePublicKey checks canonical public material, never secret possession.
func ValidatePublicKey(role Role, publicKey []byte) error {
	if role < RecipientView || role > ManagerAuthority {
		return ErrDomain
	}
	if role == RecipientSpend {
		return validateOwner(publicKey)
	}
	if role.viewing() {
		_, err := envelope.ParsePublicKey(publicKey)
		return err
	}
	if len(publicKey) != crypto.Falcon1024PublicKeySize || publicKey[0] != 10 {
		return ErrKey
	}
	return validateFalconCoefficients(publicKey[1:])
}
