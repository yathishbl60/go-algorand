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

func (d Domain) Validate() error {
	if d.Network == [32]byte{} || d.Asset == 0 || d.Policy == [64]byte{} || d.Account == [32]byte{} {
		return ErrDomain
	}
	if d.Role < RecipientView || d.Role > ManagerAuthority {
		return ErrDomain
	}
	return nil
}

func (d Domain) bytes() []byte {
	b := append([]byte(nil), d.Network[:]...)
	b = binary.BigEndian.AppendUint64(b, d.Asset)
	b = append(b, d.Policy[:]...)
	b = append(b, d.Account[:]...)
	return append(b, byte(d.Role))
}

func decodeDomain(b []byte) (Domain, error) {
	if len(b) != 137 {
		return Domain{}, ErrDomain
	}
	var d Domain
	copy(d.Network[:], b[:32])
	d.Asset = binary.BigEndian.Uint64(b[32:40])
	copy(d.Policy[:], b[40:104])
	copy(d.Account[:], b[104:136])
	d.Role = Role(b[136])
	return d, d.Validate()
}
