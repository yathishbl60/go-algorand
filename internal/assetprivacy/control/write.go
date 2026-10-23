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

package control

// Set persists only caller-authorized control state; expectedEpoch rejects stale actions.
func (s *Store) Set(frozen bool, expectedEpoch, round uint64) (Record, error) {
	record, err := s.nextRecord(frozen, expectedEpoch, round)
	if err != nil {
		return Record{}, err
	}
	encoded, err := s.encodeNew(record, round)
	if err != nil {
		return Record{}, err
	}
	err = s.kv.Apply([]Write{{s.domain.epochKey(record.Epoch), encoded}, s.pointer(record.Epoch)})
	if err != nil {
		return Record{}, err
	}
	return record, nil
}

func (s *Store) nextRecord(frozen bool, expectedEpoch, round uint64) (Record, error) {
	before, exists, err := s.Current(round)
	if err != nil {
		return Record{}, err
	}
	if (!exists && expectedEpoch != 0) || (exists && expectedEpoch != before.Epoch) {
		return Record{}, ErrStale
	}
	if expectedEpoch == ^uint64(0) {
		return Record{}, ErrEpoch
	}
	return Record{s.domain, expectedEpoch + 1, round, frozen}, nil
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
