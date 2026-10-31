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

func firstKey(kv Getter, record Record, value node, found bool) (string, bool, error) {
	if !found || value.Prev.Exists || value.Next.Exists != (record.Items > 1) {
		return "", false, ErrIndex
	}
	data, exists, err := kv.Get(value.Key)
	if err != nil {
		return "", false, err
	}
	if !exists || uint64(len(data)) != value.ValueBytes {
		return "", false, ErrIndex
	}
	size, err := footprint(value.Key, value.ValueBytes)
	if err != nil || size > record.Bytes || record.Items == 1 && size != record.Bytes {
		return "", false, ErrIndex
	}
	return value.Key, true, nil
}
