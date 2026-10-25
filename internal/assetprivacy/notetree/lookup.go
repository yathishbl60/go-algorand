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

// CheckNew checks the authoritative creation map, not a cryptographic nonmembership proof.
func (s *Store) CheckNew(commitment tree.Hash) error {
	current, err := s.current()
	if err != nil {
		return err
	}
	if commitment == (tree.Hash{}) {
		return tree.ErrCommitment
	}
	_, exists, err := s.lookup(current, commitment)
	if err != nil {
		return err
	}
	if exists {
		return ErrDuplicate
	}
	return nil
}

func (s *Store) lookup(current *tree.Tree, commitment tree.Hash) (uint32, bool, error) {
	encoded, exists, err := s.kv.Get(commitmentKey(s.domain, commitment))
	if err != nil || !exists {
		return 0, exists, err
	}
	index, err := decodeIndex(encoded, s.domain, commitment)
	if err != nil {
		return 0, true, err
	}
	witness, err := current.Witness(index)
	if err != nil {
		return 0, true, err
	}
	if witness.Leaf != commitment {
		return 0, true, ErrEncoding
	}
	return index, true, nil
}
