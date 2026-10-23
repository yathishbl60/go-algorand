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

// AppendAtRound preserves the round's first pre-append root in the bounded ring.
// The trusted consensus slot count must be positive and cannot be changed in place.
func (t *Tree) AppendAtRound(leaf Hash, round, slots uint64) (uint32, Hash, error) {
	path, err := t.appendPath(leaf)
	if err != nil {
		return 0, Hash{}, err
	}
	history, err := t.prepareHistory(round, slots)
	if err != nil {
		return 0, Hash{}, err
	}
	records, root := t.appendRecords(leaf, path)
	if err = t.kv.Apply(append(history, records...)); err != nil {
		return 0, Hash{}, err
	}
	index := uint32(t.count)
	t.root, t.count = root, t.count+1
	return index, root, nil
}

func (t *Tree) prepareHistory(round, slots uint64) ([]Record, error) {
	head, _, err := t.readHistory(round, slots)
	if err != nil {
		return nil, err
	}
	if head != nil && head.Round == round {
		return nil, nil
	}
	snapshot := historySnapshot{t.root, t.count, round, slots}
	records := []Record{t.historyRecord(t.domain.historyHeadKey(), snapshot)}
	return append(records, t.historyRecord(t.domain.historySlotKey(round%slots), snapshot)), nil
}
