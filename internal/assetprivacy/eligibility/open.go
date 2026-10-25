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

package eligibility

import "github.com/algorand/go-algorand/internal/assetprivacy/tree"

func Open(kv tree.KV, domain tree.Domain) (*Store, error) {
	if _, err := tree.Open(treeKV{kv}, domain); err != nil {
		return nil, err
	}
	return &Store{kv: kv, domain: domain}, nil
}

// Snapshot returns only the current root; no note-root history is accepted here.
func (s *Store) Snapshot() (Root, uint64, error) {
	current, err := tree.Open(treeKV{s.kv}, s.domain)
	if err != nil {
		return Root{}, 0, err
	}
	return Root{Digest: current.Root()}, current.Count(), nil
}
