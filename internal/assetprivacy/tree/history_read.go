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

func (t *Tree) readHistory(round, slots uint64) (*historySnapshot, []historySnapshot, error) {
	if slots == 0 {
		return nil, nil, ErrHistory
	}
	encoded, exists, err := t.kv.Get(t.domain.historyHeadKey())
	if err != nil {
		return nil, nil, err
	}
	var head *historySnapshot
	if exists {
		value, decodeErr := t.decodeHistory(encoded, round, slots)
		if decodeErr != nil {
			return nil, nil, decodeErr
		}
		head = &value
	}
	snapshots, err := t.readHistorySlots(head, round, slots)
	return head, snapshots, err
}

func (t *Tree) readHistorySlots(head *historySnapshot, round, slots uint64) ([]historySnapshot, error) {
	var snapshots []historySnapshot
	for slot := uint64(0); slot < slots; slot++ {
		entry, exists, err := t.readHistorySlot(slot, round, slots)
		if err != nil {
			return nil, err
		}
		if !exists {
			continue
		}
		if err = entry.checkSlot(head, slot); err != nil {
			return nil, err
		}
		snapshots = append(snapshots, entry)
	}
	if head != nil && !containsSnapshot(snapshots, *head) {
		return nil, ErrHistory
	}
	return snapshots, nil
}

func containsSnapshot(snapshots []historySnapshot, expected historySnapshot) bool {
	for _, snapshot := range snapshots {
		if snapshot == expected {
			return true
		}
	}
	return false
}
