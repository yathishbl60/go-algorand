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

func (s *Store) prepare(after, before Record, exists bool) ([]tree.Record, error) {
	staged := newStage(s.kv)
	current, err := tree.OpenMutable(treeKV{staged}, s.domain)
	if err != nil {
		return nil, err
	}
	leaf, err := LeafHash(after)
	if err != nil {
		return nil, err
	}
	indexed, err := s.changeTree(current, after, before, exists, leaf)
	if err != nil {
		return nil, err
	}
	return staged.withIndex(indexed)
}
