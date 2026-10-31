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

package inventory

func (w *walk) step(kv Getter, asset uint64) error {
	if w.seen[w.current.ID] || uint64(len(w.keys)) >= w.record.Items {
		return ErrIndex
	}
	value, exists, err := readNode(kv, asset, w.current.ID)
	if err != nil {
		return err
	}
	if !exists || value.Prev != w.previous {
		return ErrIndex
	}
	if err = w.data(kv, value); err != nil {
		return err
	}
	w.seen[w.current.ID] = true
	w.keys = append(w.keys, value.Key)
	w.previous, w.current = w.current, value.Next
	return nil
}

func (w *walk) data(kv Getter, value node) error {
	data, exists, err := kv.Get(value.Key)
	if err != nil {
		return err
	}
	if !exists || uint64(len(data)) != value.ValueBytes {
		return ErrIndex
	}
	size, err := footprint(value.Key, value.ValueBytes)
	if err != nil || size > w.record.Bytes-w.bytes {
		return ErrIndex
	}
	w.bytes += size
	return nil
}

func (w *walk) finish() error {
	if !w.current.Exists && (uint64(len(w.keys)) != w.record.Items || w.bytes != w.record.Bytes) {
		return ErrIndex
	}
	if w.current.Exists && (uint64(len(w.keys)) >= w.record.Items || w.seen[w.current.ID]) {
		return ErrIndex
	}
	if !w.current.Exists && w.record.Items != 0 && w.previous.ID != w.record.Tail {
		return ErrIndex
	}
	return nil
}
