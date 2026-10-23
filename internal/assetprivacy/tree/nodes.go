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

func (t *Tree) nodeRecord(level byte, left, right Hash) Record {
	encoded := append([]byte{1, level}, left[:]...)
	encoded = append(encoded, right[:]...)
	return Record{t.domain.nodeKey(t.domain.NodeHash(level, left, right)), encoded}
}

func (t *Tree) children(root Hash, level byte) (Hash, Hash, error) {
	if root == t.empty[level+1] {
		return t.empty[level], t.empty[level], nil
	}
	encoded, exists, err := t.kv.Get(t.domain.nodeKey(root))
	if err != nil {
		return Hash{}, Hash{}, err
	}
	if !exists {
		return Hash{}, Hash{}, ErrMissing
	}
	return t.decodeNode(encoded, root, level)
}

func (t *Tree) decodeNode(encoded []byte, root Hash, level byte) (Hash, Hash, error) {
	if len(encoded) != 130 || encoded[0] != 1 || encoded[1] != level {
		return Hash{}, Hash{}, ErrEncoding
	}
	var left, right Hash
	copy(left[:], encoded[2:66])
	copy(right[:], encoded[66:])
	if t.domain.NodeHash(level, left, right) != root {
		return Hash{}, Hash{}, ErrEncoding
	}
	return left, right, nil
}
