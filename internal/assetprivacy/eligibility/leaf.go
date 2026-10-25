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

package eligibility

import (
	"crypto/sha3"

	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

// LeafHash commits the exact canonical derived snapshot under a separate hash domain.
func LeafHash(record Record) (tree.Hash, error) {
	encoded, err := Encode(record)
	if err != nil {
		return tree.Hash{}, err
	}
	message := append([]byte("Algorand/AssetPrivacy/Candidate1/Eligibility/"), encoded...)
	return sha3.Sum512(message), nil
}
