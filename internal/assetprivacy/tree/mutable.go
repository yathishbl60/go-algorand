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

// Mutable grants replacement in addition to the append-only Tree API.
// Callers must serialize KV access and keep this capability out of note storage.
type Mutable struct{ *Tree }

func OpenMutable(kv KV, domain Domain) (*Mutable, error) {
	t, err := Open(kv, domain)
	if err != nil {
		return nil, err
	}
	return &Mutable{t}, nil
}

// Replace atomically changes one existing leaf without changing its stable index.
func (t *Mutable) Replace(index uint32, before, after Hash) (Hash, error) {
	if after == t.empty[0] {
		return Hash{}, ErrCommitment
	}
	path, err := t.Witness(index)
	if err != nil {
		return Hash{}, err
	}
	if path.Leaf != before {
		return Hash{}, ErrCommitment
	}
	records, root := t.replacementRecords(after, path)
	if err = t.kv.Apply(records); err != nil {
		return Hash{}, err
	}
	t.root = root
	return root, nil
}
