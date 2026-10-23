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

// Put only persists preauthorized registration state. It is not a verifier.
func (s *Store) Put(record Record, round uint64) error {
	if err := record.Validate(); err != nil {
		return err
	}
	if record.Domain != s.domain || record.Created != round {
		return ErrRecord
	}
	before, exists, err := s.Current(round)
	if err != nil {
		return err
	}
	if err = checkNext(before, exists, record); err != nil {
		return err
	}
	encoded, err := s.encodeNew(record, round)
	if err != nil {
		return err
	}
	return s.kv.Apply([]Write{{s.domain.epochKey(record.Epoch), encoded}, s.pointer(record.Epoch)})
}

func checkNext(before Record, exists bool, after Record) error {
	if !exists && after.Epoch != 1 {
		return ErrEpoch
	}
	if exists && (before.Epoch == ^uint64(0) || after.Epoch != before.Epoch+1 || after.Created < before.Created) {
		return ErrEpoch
	}
	return nil
}

func (s *Store) encodeNew(record Record, round uint64) ([]byte, error) {
	_, exists, err := s.Epoch(record.Epoch, round)
	if err != nil {
		return nil, err
	}
	if exists {
		return nil, ErrHistory
	}
	return Encode(record)
}
