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

func (s *Store) changeTree(current *tree.Mutable, after, before keys.Record, exists bool, leaf tree.Hash) (indexRecord, error) {
	indexed, found, err := s.readIndex(after.Domain)
	if err != nil {
		return indexRecord{}, err
	}
	if found != exists {
		return indexRecord{}, ErrHistory
	}
	if exists {
		return rotate(current, indexed, after, before, leaf)
	}
	index, _, err := current.Append(leaf)
	return indexRecord{after.Domain, index, leaf, after.Epoch}, err
}

func rotate(current *tree.Mutable, indexed indexRecord, after, before keys.Record, leaf tree.Hash) (indexRecord, error) {
	old, err := LeafHash(before)
	if err != nil {
		return indexRecord{}, err
	}
	if indexed.leaf != old || indexed.epoch != before.Epoch {
		return indexRecord{}, ErrHistory
	}
	if _, err = current.Replace(indexed.index, old, leaf); err != nil {
		return indexRecord{}, err
	}
	return indexRecord{after.Domain, indexed.index, leaf, after.Epoch}, nil
}
