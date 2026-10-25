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

package notetree

import "github.com/algorand/go-algorand/internal/assetprivacy/tree"

// AppendAtRound atomically couples nodes/history, global uniqueness and generation marker.
func (s *Store) AppendAtRound(commitment tree.Hash, round, slots uint64) (uint32, tree.Hash, error) {
	if err := s.CheckNew(commitment); err != nil {
		return 0, tree.Hash{}, err
	}
	staged := newStage(s.kv)
	current, err := s.loadCurrent(staged)
	if err != nil {
		return 0, tree.Hash{}, err
	}
	index, root, err := current.AppendAtRound(commitment, round, slots)
	if err != nil {
		return 0, tree.Hash{}, err
	}
	s.stageCreation(staged, commitment, index, root, current.Count())
	if err = s.kv.Apply(staged.records); err != nil {
		return 0, tree.Hash{}, err
	}
	s.root, s.count = root, current.Count()
	return index, root, nil
}

func (s *Store) stageCreation(staged *stage, commitment tree.Hash, index uint32, root tree.Hash, count uint64) {
	staged.add(tree.Record{Key: commitmentKey(s.domain, commitment), Value: indexValue(s.domain, commitment, index)})
	staged.add(tree.Record{Key: markerKey(s.domain), Value: marker(s.domain, root, count)})
}
