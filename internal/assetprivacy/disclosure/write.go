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

// Set persists a preauthorized assignment using the authoritative current clawback.
// A zero current clawback preserves prior controller viewing access only.
func (s *Store) Set(auditor, currentClawback [32]byte, expectedEpoch, round uint64) (Record, error) {
	record, err := s.next(auditor, currentClawback, expectedEpoch, round)
	if err != nil {
		return Record{}, err
	}
	encoded, err := Encode(record)
	if err != nil {
		return Record{}, err
	}
	if err = s.kv.Apply([]Write{{s.domain.key(), encoded}}); err != nil {
		return Record{}, err
	}
	return record, nil
}

func (s *Store) next(auditor, clawback [32]byte, expectedEpoch, round uint64) (Record, error) {
	if (s.flags&AuditorRequired != 0) != (auditor != [32]byte{}) {
		return Record{}, ErrTarget
	}
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
	controller := s.controller(before, exists, clawback)
	record := Record{s.domain, s.flags, auditor, controller, expectedEpoch + 1, round}
	return record, record.Validate()
}

func (s *Store) controller(before Record, exists bool, clawback [32]byte) [32]byte {
	if s.flags&ControllerRequired == 0 {
		return [32]byte{}
	}
	if clawback != [32]byte{} {
		return clawback
	}
	if exists {
		return before.Controller
	}
	return [32]byte{}
}
