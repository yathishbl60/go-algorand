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

// AcceptRoot recognizes the current state or a recorded root younger than slots.
// Count is bound alongside root. Root knowledge alone never authorizes a spend.
func (t *Tree) AcceptRoot(root Hash, count, currentRound, slots uint64) (bool, error) {
	if err := t.checkCurrent(); err != nil {
		return false, err
	}
	_, snapshots, err := t.readHistory(currentRound, slots)
	if err != nil {
		return false, err
	}
	if root == t.root && count == t.count {
		return true, nil
	}
	for _, snapshot := range snapshots {
		if currentRound-snapshot.Round < slots && snapshot.Root == root && snapshot.Count == count {
			return true, nil
		}
	}
	return false, nil
}
