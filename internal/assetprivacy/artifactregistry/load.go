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

package artifactregistry

// Load checks the independently pinned exact registry before publishing owned state.
// A fixed-size snapshot prevents subsequent caller-buffer mutations affecting reads.
func Load(encoded []byte, pin Pin) (*Registry, error) {
	if err := pin.validate(); err != nil {
		return nil, err
	}
	if len(encoded) != EncodedSize {
		return nil, ErrEncoding
	}
	var snapshot [EncodedSize]byte
	copy(snapshot[:], encoded)
	if err := checkInput(snapshot[:], pin); err != nil {
		return nil, err
	}
	entries, err := readEntries(snapshot[:])
	if err != nil {
		return nil, err
	}
	return &Registry{pin: pin, entries: entries}, nil
}

func readEntries(encoded []byte) ([EntryCount]Entry, error) {
	var entries [EntryCount]Entry
	for index, expected := range layouts() {
		start := headerSize + index*entrySize
		entry, err := decodeEntry(encoded[start:start+entrySize], expected)
		if err != nil {
			return [EntryCount]Entry{}, err
		}
		entries[index] = entry
	}
	return entries, nil
}
