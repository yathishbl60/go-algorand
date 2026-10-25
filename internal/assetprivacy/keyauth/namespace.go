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

package keyauth

import "github.com/algorand/go-algorand/internal/assetprivacy/tree"

type treeKV struct{ kv tree.KV }

func (s treeKV) Get(key string) ([]byte, bool, error) {
	return s.kv.Get(namespace + key)
}

func (s treeKV) Apply(records []tree.Record) error {
	prefixed := make([]tree.Record, len(records))
	for i, record := range records {
		prefixed[i] = tree.Record{Key: namespace + record.Key, Value: append([]byte(nil), record.Value...)}
	}
	return s.kv.Apply(prefixed)
}
