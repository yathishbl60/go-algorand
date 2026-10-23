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

func (t *Tree) readHistorySlot(slot, round, slots uint64) (historySnapshot, bool, error) {
	encoded, exists, err := t.kv.Get(t.domain.historySlotKey(slot))
	if err != nil || !exists {
		return historySnapshot{}, exists, err
	}
	snapshot, err := t.decodeHistory(encoded, round, slots)
	return snapshot, true, err
}

func (h historySnapshot) checkSlot(head *historySnapshot, slot uint64) error {
	if head == nil || h.Round%h.Slots != slot || h.Round > head.Round {
		return ErrHistory
	}
	if h.Round == head.Round && h != *head {
		return ErrHistory
	}
	if h.Round < head.Round && h.Count >= head.Count {
		return ErrHistory
	}
	return nil
}
