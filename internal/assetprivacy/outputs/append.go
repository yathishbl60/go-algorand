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

import "github.com/algorand/go-algorand/internal/assetprivacy/tree"

// CheckAppend checks persistence preconditions only. It grants no spending authority.
func (s *Store) CheckAppend(r Record) error {
	if err := s.checkNext(r); err != nil {
		return err
	}
	if err := s.checkCurrent(); err != nil {
		return err
	}
	return s.checkVacant(r.Index)
}

func (s *Store) checkNext(r Record) error {
	if r.Domain != s.domain {
		return ErrDomain
	}
	if r.Disclosures != s.disclosures {
		return ErrDisclosures
	}
	if s.count == tree.Capacity || uint64(r.Index) != s.count {
		return ErrIndex
	}
	if r.Round < s.round {
		return ErrState
	}
	return r.Validate()
}

func (s *Store) checkVacant(index uint32) error {
	_, exists, err := s.kv.Get(s.recordKey(index))
	if err != nil {
		return err
	}
	if exists {
		return ErrState
	}
	return nil
}

// Append atomically writes the immutable output and advances metadata.
// Native callers must couple this to the commitment append in their parent COW.
func (s *Store) Append(r Record) error {
	if err := s.CheckAppend(r); err != nil {
		return err
	}
	encoded, err := r.Encode()
	if err != nil {
		return err
	}
	records := []tree.Record{{Key: s.recordKey(r.Index), Value: encoded}, s.metadata(s.count+1, r.Round)}
	if err = s.kv.Apply(records); err != nil {
		return err
	}
	s.count, s.round = s.count+1, r.Round
	return nil
}
