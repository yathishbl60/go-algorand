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

package disclosure

func Open(kv KV, domain Domain, flags Flags) (*Store, error) {
	if err := domain.Validate(); err != nil {
		return nil, err
	}
	if err := flags.Validate(); err != nil {
		return nil, err
	}
	return &Store{kv: kv, domain: domain, flags: flags}, nil
}

// Current exposes canonical state without granting assignment authority.
func (s *Store) Current(round uint64) (Record, bool, error) {
	encoded, exists, err := s.kv.Get(s.domain.key())
	if err != nil || !exists {
		return Record{}, exists, err
	}
	record, err := Decode(encoded)
	if err != nil {
		return Record{}, true, err
	}
	if record.Domain != s.domain || record.Flags != s.flags {
		return Record{}, true, ErrRecord
	}
	if record.Updated > round {
		return Record{}, true, ErrRound
	}
	return record, true, nil
}
