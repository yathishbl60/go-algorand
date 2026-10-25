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

package notetree

import "github.com/algorand/go-algorand/internal/assetprivacy/tree"

type stage struct {
	parent  tree.KV
	pending map[string][]byte
	records []tree.Record
}

func newStage(parent tree.KV) *stage {
	return &stage{parent: parent, pending: make(map[string][]byte)}
}

func (s *stage) Get(key string) ([]byte, bool, error) {
	if value, exists := s.pending[key]; exists {
		return append([]byte(nil), value...), true, nil
	}
	return s.parent.Get(key)
}

func (s *stage) Apply(records []tree.Record) error {
	for _, record := range records {
		s.add(record)
	}
	return nil
}

func (s *stage) add(record tree.Record) {
	value := append([]byte(nil), record.Value...)
	s.pending[record.Key] = value
	s.records = append(s.records, tree.Record{Key: record.Key, Value: value})
}
