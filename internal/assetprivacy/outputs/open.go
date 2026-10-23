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

package outputs

import (
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

// Open uses trusted immutable policy disclosures. It does not authenticate policy.
// Only absent metadata denotes empty state; stored empty metadata is noncanonical.
func Open(kv tree.KV, domain tree.Domain, flags policy.Disclosures) (*Store, error) {
	if err := checkDomain(domain); err != nil {
		return nil, err
	}
	if err := checkDisclosures(flags); err != nil {
		return nil, err
	}
	s := &Store{kv: kv, domain: domain, disclosures: flags}
	encoded, exists, err := kv.Get(s.metadataKey())
	if err != nil {
		return nil, err
	}
	if exists {
		if err = s.decodeMetadata(encoded); err != nil {
			return nil, err
		}
	}
	return s, nil
}

func (s *Store) checkCurrent() error {
	current, err := Open(s.kv, s.domain, s.disclosures)
	if err != nil {
		return err
	}
	if s.count != current.count || s.round != current.round {
		return ErrStale
	}
	return nil
}
