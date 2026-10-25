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

func (s *Store) changeTree(current *tree.Mutable, after, before Record, exists bool, leaf tree.Hash) (indexedRecord, error) {
	indexed, found, err := s.readIndex(after.Domain, after.Updated)
	if err != nil {
		return indexedRecord{}, err
	}
	if found != exists {
		return indexedRecord{}, ErrHistory
	}
	if exists {
		return replace(current, indexed, after, before, leaf)
	}
	index, _, err := current.Append(leaf)
	return indexedRecord{index: index, record: after}, err
}

func replace(current *tree.Mutable, indexed indexedRecord, after, before Record, leaf tree.Hash) (indexedRecord, error) {
	if indexed.record != before {
		return indexedRecord{}, ErrHistory
	}
	old, err := LeafHash(before)
	if err != nil {
		return indexedRecord{}, err
	}
	if _, err = current.Replace(indexed.index, old, leaf); err != nil {
		return indexedRecord{}, err
	}
	return indexedRecord{index: indexed.index, record: after}, nil
}
