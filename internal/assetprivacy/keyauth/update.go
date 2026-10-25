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
	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

// Update persists a preauthorized current-key change and its stable index atomically.
// A prior key without an authenticated index fails closed; no implicit migration.
func (s *Store) Update(after, before keys.Record, exists bool, round uint64) error {
	if err := s.checkUpdate(after, before, exists, round); err != nil {
		return err
	}
	leaf, err := LeafHash(after)
	if err != nil {
		return err
	}
	records, err := s.prepare(after, before, exists, leaf)
	if err != nil {
		return err
	}
	return s.kv.Apply(records)
}

func (s *Store) prepare(after, before keys.Record, exists bool, leaf tree.Hash) ([]tree.Record, error) {
	staged := newStage(s.kv)
	current, err := tree.OpenMutable(treeKV{staged}, s.domain)
	if err != nil {
		return nil, err
	}
	record, err := s.changeTree(current, after, before, exists, leaf)
	if err != nil {
		return nil, err
	}
	staged.add(tree.Record{Key: indexKey(after.Domain), Value: encodeIndex(record)})
	return staged.records, nil
}
