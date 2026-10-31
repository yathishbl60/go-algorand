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

import "errors"

const MaxKeyBytes = 256
const MetadataSize = 145
const nodeHeader = 141
const prefix = "aprv:inventory:"

var ErrIndex = errors.New("invalid native privacy inventory")
var ErrMissing = errors.New("native privacy inventory is missing")
var ErrChange = errors.New("invalid native privacy inventory change")

type ID [64]byte
type link struct {
	Exists bool
	ID     ID
}
type Record struct {
	Items, Bytes uint64
	Head, Tail   ID
}
type node struct {
	Key        string
	ValueBytes uint64
	Prev, Next link
}
type Change struct {
	Key                          string
	Before, After                uint64
	Existed, Present, Initialize bool
}
type Write struct {
	Key   string
	Value []byte
}
type Getter interface {
	Get(string) ([]byte, bool, error)
}
type mutation struct {
	kv     Getter
	asset  uint64
	change Change
	record Record
	id     ID
	node   node
	writes []Write
}

func (r Record) Validate() error {
	if r.Items == 0 && (r.Bytes != 0 || r.Head != (ID{}) || r.Tail != (ID{})) {
		return ErrIndex
	}
	if r.Items != 0 && r.Bytes < r.Items {
		return ErrIndex
	}
	if r.Items != 0 && (r.Items == 1) != (r.Head == r.Tail) {
		return ErrIndex
	}
	return nil
}
