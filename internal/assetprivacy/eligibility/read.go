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

package eligibility

// Current authenticates the indexed record under the exact current tree root.
func (s *Store) Current(account [32]byte, round uint64) (Record, bool, error) {
	domain, err := s.accountDomain(account)
	if err != nil {
		return Record{}, false, err
	}
	if _, _, err = s.Snapshot(); err != nil {
		return Record{}, false, err
	}
	indexed, exists, err := s.readIndex(domain, round)
	if err != nil || !exists {
		return Record{}, exists, err
	}
	if _, err = s.currentWitness(indexed); err != nil {
		return Record{}, true, err
	}
	return indexed.record, true, nil
}
