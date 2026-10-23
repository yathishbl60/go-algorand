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

package note

import "crypto/rand"

// GenerateSpendingSecret creates an independent 256-bit private spending secret.
func GenerateSpendingSecret() ([32]byte, error) {
	var secret [32]byte
	if _, err := rand.Read(secret[:]); err != nil {
		return [32]byte{}, err
	}
	if secret == [32]byte{} {
		return [32]byte{}, ErrSecret
	}
	return secret, nil
}

// Owner derives the candidate commitment to a private spending secret.
// The proof must check this relation inside the note's network domain.
func Owner(network, secret [32]byte) ([64]byte, error) {
	if network == [32]byte{} {
		return [64]byte{}, ErrDomain
	}
	if secret == [32]byte{} {
		return [64]byte{}, ErrSecret
	}
	return digest("SpendingKey", network, secret[:]), nil
}
