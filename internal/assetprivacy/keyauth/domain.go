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

package keyauth

import (
	"encoding/binary"

	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
)

func domainBytes(domain keys.Domain) []byte {
	encoded := append([]byte(nil), domain.Network[:]...)
	encoded = binary.BigEndian.AppendUint64(encoded, domain.Asset)
	encoded = append(encoded, domain.Policy[:]...)
	encoded = append(encoded, domain.Account[:]...)
	return append(encoded, byte(domain.Role))
}

func (s *Store) checkDomain(domain keys.Domain) error {
	if err := domain.Validate(); err != nil {
		return err
	}
	if domain.Network != s.domain.Genesis || domain.Asset != s.domain.Asset || domain.Policy != s.domain.Policy {
		return ErrDomain
	}
	return nil
}

func indexKey(domain keys.Domain) string { return namespace + "i" + string(domainBytes(domain)) }
