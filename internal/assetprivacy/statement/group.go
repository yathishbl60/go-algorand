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
	"github.com/algorand/go-algorand/crypto"
	"github.com/algorand/go-algorand/data/transactions"
)

func checkGroup(group []transactions.Transaction) error {
	id := group[0].Group
	if len(group) > 1 && id.IsZero() {
		return ErrGroup
	}
	hashes, err := groupHashes(group)
	if err != nil {
		return err
	}
	if !id.IsZero() && crypto.HashObj(transactions.TxGroup{TxGroupHashes: hashes}) != id {
		return ErrGroup
	}
	return nil
}

func groupHashes(group []transactions.Transaction) ([]crypto.Digest, error) {
	hashes := make([]crypto.Digest, len(group))
	seen := make(map[crypto.Digest]bool, len(group))
	for i, tx := range group {
		if tx.Group != group[0].Group {
			return nil, ErrGroup
		}
		tx.Group = crypto.Digest{}
		hashes[i] = crypto.Digest(tx.ID())
		if seen[hashes[i]] {
			return nil, ErrGroup
		}
		seen[hashes[i]] = true
	}
	return hashes, nil
}
