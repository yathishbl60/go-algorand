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

import "encoding/binary"

func (d Domain) historySlotKey(slot uint64) string {
	return d.prefix() + "r" + string(binary.BigEndian.AppendUint64(nil, slot))
}

func (t *Tree) historyRecord(key string, snapshot historySnapshot) Record {
	encoded := t.metadata(snapshot.Root, snapshot.Count).Value
	encoded = binary.BigEndian.AppendUint64(encoded, snapshot.Round)
	encoded = binary.BigEndian.AppendUint64(encoded, snapshot.Slots)
	return Record{key, encoded}
}

func (t *Tree) decodeHistory(encoded []byte, round, slots uint64) (historySnapshot, error) {
	if len(encoded) != 185 {
		return historySnapshot{}, ErrEncoding
	}
	snapshot := &Tree{domain: t.domain, empty: t.empty}
	if err := snapshot.decodeMetadata(encoded[:169]); err != nil {
		return historySnapshot{}, err
	}
	h := historySnapshot{snapshot.root, snapshot.count, binary.BigEndian.Uint64(encoded[169:177]), binary.BigEndian.Uint64(encoded[177:])}
	if h.Slots != slots || h.Round > round || h.Count >= t.count {
		return historySnapshot{}, ErrHistory
	}
	return h, t.checkHistoryRoot(h)
}

func (t *Tree) checkHistoryRoot(snapshot historySnapshot) error {
	if snapshot.Count == 0 {
		return nil
	}
	_, _, err := t.children(snapshot.Root, Depth-1)
	return err
}
