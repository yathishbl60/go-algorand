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

package policy

import (
	"crypto/sha3"
	"encoding/binary"
)

// Identity remains stable when authorities are relinquished or keys are rotated.
func (r Record) Identity(network [32]byte, asset uint64) ([64]byte, error) {
	if err := r.Validate(); err != nil {
		return [64]byte{}, err
	}
	if network == [32]byte{} || asset == 0 {
		return [64]byte{}, ErrRecord
	}
	hash := sha3.New512()
	hash.Write([]byte("Algorand/AssetPrivacy/Candidate1/Policy/"))
	hash.Write(network[:])
	hash.Write(binary.BigEndian.AppendUint64(nil, asset))
	hash.Write([]byte{1, byte(r.Policy.Modes), byte(r.InitialCapabilities), byte(r.Policy.Disclosures)})
	hash.Write(r.Suite[:])
	hash.Write(binary.BigEndian.AppendUint64(nil, r.Created))
	var digest [64]byte
	hash.Sum(digest[:0])
	return digest, nil
}
