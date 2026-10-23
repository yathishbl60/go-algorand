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

// Append atomically persists new content-addressed nodes and count/root metadata.
// An Apply error must leave KV unchanged; the in-memory snapshot never advances.
func (t *Tree) Append(leaf Hash) (uint32, Hash, error) {
	path, err := t.appendPath(leaf)
	if err != nil {
		return 0, Hash{}, err
	}
	records, root := t.appendRecords(leaf, path)
	if err = t.kv.Apply(records); err != nil {
		return 0, Hash{}, err
	}
	index := uint32(t.count)
	t.root, t.count = root, t.count+1
	return index, root, nil
}

func (t *Tree) appendRecords(leaf Hash, path Witness) ([]Record, Hash) {
	records := make([]Record, 0, Depth+1)
	node := leaf
	for level := byte(0); level < Depth; level++ {
		left, right := node, path.Siblings[level]
		if path.Index&(1<<level) != 0 {
			left, right = right, left
		}
		records = append(records, t.nodeRecord(level, left, right))
		node = t.domain.NodeHash(level, left, right)
	}
	return append(records, t.metadata(node, t.count+1)), node
}

func (t *Tree) appendPath(leaf Hash) (Witness, error) {
	if leaf == t.empty[0] {
		return Witness{}, ErrCommitment
	}
	if t.count >= Capacity {
		return Witness{}, ErrCapacity
	}
	if err := t.checkCurrent(); err != nil {
		return Witness{}, err
	}
	path, err := t.path(uint32(t.count))
	if err != nil {
		return Witness{}, err
	}
	if path.Leaf != t.empty[0] {
		return Witness{}, ErrEncoding
	}
	return path, nil
}
