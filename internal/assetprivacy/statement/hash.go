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
	"encoding/binary"
	"hash"

	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/protocol"
)

func hashBodies(digest hash.Hash, group []transactions.Transaction, limits Limits) ([64]byte, error) {
	remaining := limits.MaxGroupEstimate
	for i := range group {
		encoded := protocol.Encode(&group[i])
		if uint64(len(encoded)) > limits.MaxBodyEstimate || uint64(len(encoded)) > remaining {
			return [64]byte{}, ErrLimit
		}
		remaining -= uint64(len(encoded))
		digest.Write(binary.BigEndian.AppendUint64(nil, uint64(len(encoded))))
		digest.Write(encoded)
	}
	var out [64]byte
	digest.Sum(out[:0])
	return out, nil
}
