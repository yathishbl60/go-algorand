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

func readNode(kv Getter, asset uint64, id ID) (node, bool, error) {
	encoded, exists, err := kv.Get(nodeKey(asset, id))
	if err != nil || !exists {
		return node{}, exists, err
	}
	value, err := decodeNode(encoded)
	if err != nil {
		return node{}, true, err
	}
	if err = validateKey(asset, value.Key); err != nil || identify(value.Key) != id {
		return node{}, true, ErrIndex
	}
	return value, true, nil
}

func First(kv Getter, asset uint64) (string, bool, error) {
	record, exists, err := Read(kv, asset)
	if err != nil || record.Items == 0 && exists {
		return "", false, err
	}
	if !exists {
		return "", false, ErrMissing
	}
	value, found, err := readNode(kv, asset, record.Head)
	if err != nil {
		return "", false, err
	}
	return firstKey(kv, record, value, found)
}
