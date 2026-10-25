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

func (t *Mutable) replacementRecords(leaf Hash, path Witness) ([]Record, Hash) {
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
	return append(records, t.metadata(node, t.count)), node
}
