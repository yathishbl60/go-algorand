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

package registrationproof

import (
	"fmt"

	"github.com/algorand/go-algorand/crypto"
	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
)

// Verify proves only possession of this Falcon1024 key over the typed group.
// The caller must first check scheme enablement, current policy/principal,
// expected epoch, fees and work limits, then commit state atomically on success.
func Verify(publicKey []byte, group [64]byte, frame []byte) error {
	message, err := Message(group)
	if err != nil {
		return err
	}
	if err = keys.ValidatePublicKey(keys.ManagerAuthority, publicKey); err != nil {
		return err
	}
	signature, err := Decode(frame)
	if err != nil {
		return err
	}
	return verifySignature(publicKey, message, signature)
}

func verifySignature(publicKey, message, signature []byte) error {
	var verifier crypto.Falcon1024Verifier
	copy(verifier.PublicKey[:], publicKey)
	if err := verifier.VerifyBytes(message, crypto.Falcon1024Signature(signature)); err != nil {
		return fmt.Errorf("%w: %w", crypto.ErrPQFalcon1024SigInvalid, err)
	}
	return nil
}
