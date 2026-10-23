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

package keys

func Open(kv KV, domain Domain) (*Store, error) {
	if err := domain.Validate(); err != nil {
		return nil, err
	}
	return &Store{kv, domain}, nil
}

func (s *Store) Epoch(epoch, round uint64) (Record, bool, error) {
	if epoch == 0 {
		return Record{}, false, ErrEpoch
	}
	encoded, exists, err := s.kv.Get(s.domain.epochKey(epoch))
	if err != nil || !exists {
		return Record{}, exists, err
	}
	record, err := Decode(encoded)
	if err != nil {
		return Record{}, true, err
	}
	if record.Domain != s.domain || record.Epoch != epoch || record.Created > round {
		return Record{}, true, ErrRecord
	}
	return record, true, nil
}

func (s *Store) Current(round uint64) (Record, bool, error) {
	epoch, exists, err := s.readPointer()
	if err != nil || !exists {
		return Record{}, exists, err
	}
	record, exists, err := s.Epoch(epoch, round)
	if err != nil {
		return Record{}, true, err
	}
	if !exists {
		return Record{}, true, ErrHistory
	}
	return record, true, nil
}
