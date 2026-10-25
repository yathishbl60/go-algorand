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

import (
	"crypto/sha3"
	"encoding/binary"
)

// Commitment binds a complete opening and its network domain.
func (n Note) Commitment(network [32]byte) ([64]byte, error) {
	encoded, err := n.Encode()
	if err != nil {
		return [64]byte{}, err
	}
	defer clear(encoded)
	if network == [32]byte{} {
		return [64]byte{}, ErrDomain
	}
	return digest("Commitment", network, encoded), nil
}

// Nullifier V2 binds the serial and canonical commitment for holder/recovery paths.
// Knowledge of this marker or its serial alone is never spending authorization.
func (n Note) Nullifier(network [32]byte) ([64]byte, error) {
	commitment, err := n.Commitment(network)
	if err != nil {
		return [64]byte{}, err
	}
	encoded := binary.BigEndian.AppendUint64(nil, n.Asset)
	encoded = append(encoded, n.Policy[:]...)
	encoded = append(encoded, n.Serial[:]...)
	encoded = append(encoded, commitment[:]...)
	defer clear(encoded)
	return digest("NullifierV2", network, encoded), nil
}

func digest(role string, network [32]byte, encoded []byte) [64]byte {
	hash := sha3.New512()
	hash.Write([]byte("Algorand/AssetPrivacy/Candidate1/" + role + "/"))
	hash.Write(network[:])
	hash.Write(encoded)
	var out [64]byte
	hash.Sum(out[:0])
	return out
}
