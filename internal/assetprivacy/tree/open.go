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

package tree

// Open loads canonical metadata; absence denotes the deterministic empty tree.
// It does not authorize the domain policy or accept historical roots for spending.
func Open(kv KV, domain Domain) (*Tree, error) {
	if err := domain.validate(); err != nil {
		return nil, err
	}
	t := &Tree{kv: kv, domain: domain, empty: domain.emptyNodes()}
	t.root = t.empty[Depth]
	encoded, exists, err := kv.Get(domain.metadataKey())
	if err != nil {
		return nil, err
	}
	if exists {
		if err = t.decodeMetadata(encoded); err != nil {
			return nil, err
		}
	}
	return t, nil
}

func (t *Tree) checkCurrent() error {
	current, err := Open(t.kv, t.domain)
	if err != nil {
		return err
	}
	if current.root != t.root || current.count != t.count {
		return ErrStale
	}
	return nil
}
