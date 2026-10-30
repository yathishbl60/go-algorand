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

func (r Record) Validate() error {
	if err := r.Domain.Validate(); err != nil {
		return err
	}
	if r.Epoch == 0 {
		return ErrEpoch
	}
	return ValidatePublicKey(r.Domain.Role, r.PublicKey)
}

func (r Role) viewing() bool {
	return r == RecipientView || r == AuditorView || r == ControllerView
}

// UsesStarkPossession selects owner or ML-KEM secret-possession verification.
func (r Role) UsesStarkPossession() bool {
	return r == RecipientSpend || r.viewing()
}

// UsesFalconPossession selects only the three administrative registration roles.
func (r Role) UsesFalconPossession() bool {
	return r >= FreezeAuthority && r <= ManagerAuthority
}

// Follows pinned Falcon modq_decode: packed 14-bit values must be below 12289.
// This checks syntax only; full signature/possession verification is mandatory.
func validateFalconCoefficients(encoded []byte) error {
	var accumulator uint32
	bits := uint(0)
	for _, value := range encoded {
		accumulator = accumulator<<8 | uint32(value)
		bits += 8
		if bits >= 14 {
			bits -= 14
			if (accumulator>>bits)&0x3fff >= 12289 {
				return ErrKey
			}
		}
	}
	return nil
}

// Recipient spending uses the existing SHA3 ownership-preimage relation.
func validateOwner(encoded []byte) error {
	if len(encoded) != 64 {
		return ErrKey
	}
	var owner [64]byte
	copy(owner[:], encoded)
	if owner == [64]byte{} {
		return ErrKey
	}
	return nil
}
