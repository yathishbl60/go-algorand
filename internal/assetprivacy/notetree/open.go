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

// Open rejects nonempty unguarded experimental trees; it never infers migration.
func Open(kv tree.KV, domain tree.Domain) (*Store, error) {
	current, err := load(kv, domain)
	if err != nil {
		return nil, err
	}
	return &Store{kv: kv, domain: domain, root: current.Root(), count: current.Count()}, nil
}

func load(kv tree.KV, domain tree.Domain) (*tree.Tree, error) {
	current, err := tree.Open(kv, domain)
	if err != nil {
		return nil, err
	}
	encoded, exists, err := kv.Get(markerKey(domain))
	if err != nil {
		return nil, err
	}
	if !exists {
		if current.Count() != 0 {
			return nil, ErrGeneration
		}
		return current, nil
	}
	if err = checkMarker(encoded, domain, current); err != nil {
		return nil, err
	}
	return current, nil
}

func (s *Store) current() (*tree.Tree, error) {
	return s.loadCurrent(s.kv)
}

func (s *Store) loadCurrent(kv tree.KV) (*tree.Tree, error) {
	current, err := load(kv, s.domain)
	if err != nil {
		return nil, err
	}
	if current.Root() != s.root || current.Count() != s.count {
		return nil, tree.ErrStale
	}
	return current, nil
}
