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

package disclosure

import "encoding/binary"

func (d Domain) Validate() error {
	if d.Network == [32]byte{} || d.Asset == 0 || d.Policy == [64]byte{} {
		return ErrDomain
	}
	return nil
}

func (d Domain) bytes() []byte {
	encoded := append([]byte(nil), d.Network[:]...)
	encoded = binary.BigEndian.AppendUint64(encoded, d.Asset)
	return append(encoded, d.Policy[:]...)
}

func (d Domain) key() string { return "aprv:d:" + string(binary.BigEndian.AppendUint64(nil, d.Asset)) }

func decodeDomain(encoded []byte) (Domain, error) {
	if len(encoded) != 104 {
		return Domain{}, ErrDomain
	}
	var domain Domain
	copy(domain.Network[:], encoded[:32])
	domain.Asset = binary.BigEndian.Uint64(encoded[32:40])
	copy(domain.Policy[:], encoded[40:104])
	return domain, domain.Validate()
}
