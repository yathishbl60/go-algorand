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

// New creates independent serial/blinding randomness for a proposed private output.
func New(asset uint64, policy, owner [64]byte, account [32]byte, value uint64) (Note, error) {
	n := Note{Asset: asset, Policy: policy, Owner: owner, Account: account, Value: value}
	if _, err := rand.Read(n.Serial[:]); err != nil {
		return Note{}, err
	}
	if _, err := rand.Read(n.Blinding[:]); err != nil {
		return Note{}, err
	}
	if err := n.Validate(); err != nil {
		return Note{}, err
	}
	return n, nil
}
