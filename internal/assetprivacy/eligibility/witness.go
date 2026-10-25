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

import "github.com/algorand/go-algorand/internal/assetprivacy/tree"

// Witness accepts an exact current snapshot, never an old revision or stale root.
func (s *Store) Witness(record Record, round uint64) (tree.Witness, error) {
	if err := s.checkDomain(record.Domain); err != nil {
		return tree.Witness{}, err
	}
	if err := record.Validate(); err != nil {
		return tree.Witness{}, err
	}
	if record.Updated > round {
		return tree.Witness{}, ErrRound
	}
	indexed, exists, err := s.readIndex(record.Domain, round)
	if err != nil {
		return tree.Witness{}, err
	}
	if !exists || indexed.record != record {
		return tree.Witness{}, ErrHistory
	}
	return s.currentWitness(indexed)
}

func (s *Store) currentWitness(indexed indexedRecord) (tree.Witness, error) {
	leaf, err := LeafHash(indexed.record)
	if err != nil {
		return tree.Witness{}, err
	}
	current, err := tree.Open(treeKV{s.kv}, s.domain)
	if err != nil {
		return tree.Witness{}, err
	}
	witness, err := current.Witness(indexed.index)
	if err != nil {
		return tree.Witness{}, err
	}
	if witness.Leaf != leaf {
		return tree.Witness{}, ErrHistory
	}
	return witness, nil
}
