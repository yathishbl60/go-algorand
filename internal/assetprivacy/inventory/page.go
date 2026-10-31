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

const MaxPageKeys = 64

type walk struct {
	record   Record
	current  link
	previous link
	bytes    uint64
	seen     map[ID]bool
	keys     []string
}

func Keys(kv Getter, asset uint64, maximum int) ([]string, error) {
	if maximum < 1 || maximum > MaxPageKeys {
		return nil, ErrChange
	}
	record, exists, err := Read(kv, asset)
	if err != nil {
		return nil, err
	}
	if !exists {
		return nil, ErrMissing
	}
	state := walk{record: record, current: link{record.Items != 0, record.Head}, seen: make(map[ID]bool)}
	for len(state.keys) < maximum && state.current.Exists {
		if err = state.step(kv, asset); err != nil {
			return nil, err
		}
	}
	return state.keys, state.finish()
}
