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

package tree

// Witness reads a current-tree membership path, with private index directions.
func (t *Tree) Witness(index uint32) (Witness, error) {
	if uint64(index) >= t.count {
		return Witness{}, ErrIndex
	}
	if err := t.checkCurrent(); err != nil {
		return Witness{}, err
	}
	witness, err := t.path(index)
	if err != nil {
		return Witness{}, err
	}
	if witness.Leaf == t.empty[0] {
		return Witness{}, ErrEncoding
	}
	return witness, nil
}

func (t *Tree) path(index uint32) (Witness, error) {
	witness := Witness{Index: index, Root: t.root, Count: t.count}
	node := t.root
	for level := Depth - 1; level >= 0; level-- {
		left, right, err := t.children(node, byte(level))
		if err != nil {
			return Witness{}, err
		}
		node, witness.Siblings[level] = left, right
		if index&(1<<level) != 0 {
			node, witness.Siblings[level] = right, left
		}
	}
	witness.Leaf = node
	return witness, nil
}
