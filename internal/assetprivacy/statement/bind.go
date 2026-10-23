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

package statement

import (
	"crypto/sha3"
	"encoding/binary"

	"github.com/algorand/go-algorand/data/transactions"
)

// Bind hashes canonical transaction bodies in final order, excluding proof/signature witnesses.
// group must already satisfy ordinary consensus/encoding bounds. This function also
// checks group IDs, duplicate bodies, genesis hashes and encoding resource limits.
func Bind(genesis [32]byte, suite [64]byte, target uint32, group []transactions.Transaction, limits Limits) ([64]byte, error) {
	if genesis == [32]byte{} || suite == [64]byte{} {
		return [64]byte{}, ErrDomain
	}
	if err := preflight(genesis, target, group, limits); err != nil {
		return [64]byte{}, err
	}
	if err := checkGroup(group); err != nil {
		return [64]byte{}, err
	}
	digest := sha3.New512()
	digest.Write([]byte("Algorand/AssetPrivacy/Candidate1/Group/"))
	digest.Write(genesis[:])
	digest.Write(suite[:])
	digest.Write(binary.BigEndian.AppendUint32(nil, target))
	digest.Write(binary.BigEndian.AppendUint32(nil, uint32(len(group))))
	return hashBodies(digest, group, limits)
}
