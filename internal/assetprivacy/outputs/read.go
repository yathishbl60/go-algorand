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

// Read returns an owned public record and rejects gaps or swapped records.
func (s *Store) Read(index uint32) (Record, error) {
	if uint64(index) >= s.count {
		return Record{}, ErrIndex
	}
	encoded, exists, err := s.kv.Get(s.recordKey(index))
	if err != nil {
		return Record{}, err
	}
	if !exists {
		return Record{}, ErrState
	}
	r, err := Decode(encoded)
	if err != nil {
		return Record{}, err
	}
	return r, s.checkRecord(r, index)
}

func (s *Store) checkRecord(r Record, index uint32) error {
	if r.Domain != s.domain || r.Index != index || r.Round > s.round {
		return ErrState
	}
	if r.Disclosures != s.disclosures {
		return ErrDisclosures
	}
	return nil
}

// Page returns an immutable prefix bounded by this snapshot's count.
// Callers must serialize KV access; no live-cursor or silent-gap behavior is used.
func (s *Store) Page(start uint64, limit uint32) (Page, error) {
	if limit == 0 || limit > MaxPageSize || start > s.count {
		return Page{}, ErrPage
	}
	end := min(s.count, start+uint64(limit))
	page := Page{Records: make([]Record, 0, end-start), Next: end, Count: s.count}
	for index := start; index < end; index++ {
		r, err := s.Read(uint32(index))
		if err != nil {
			return Page{}, err
		}
		page.Records = append(page.Records, r)
	}
	return page, nil
}
