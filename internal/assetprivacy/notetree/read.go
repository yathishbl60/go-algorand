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

func (s *Store) Witness(index uint32) (tree.Witness, error) {
	current, err := s.current()
	if err != nil {
		return tree.Witness{}, err
	}
	witness, err := current.Witness(index)
	if err != nil {
		return tree.Witness{}, err
	}
	mapped, exists, err := s.lookup(current, witness.Leaf)
	if err != nil {
		return tree.Witness{}, err
	}
	if !exists || mapped != index {
		return tree.Witness{}, ErrEncoding
	}
	return witness, nil
}

func (s *Store) AcceptRoot(root tree.Hash, count, round, slots uint64) (bool, error) {
	current, err := s.current()
	if err != nil {
		return false, err
	}
	return current.AcceptRoot(root, count, round, slots)
}
