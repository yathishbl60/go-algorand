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

// Reconcile stores a PREAUTHORIZED snapshot derived by the native evaluator.
// Callers must not obtain frozen from a transaction/proof or use this as consent.
func (s *Store) Reconcile(account [32]byte, frozen bool, round uint64) (Record, error) {
	before, exists, err := s.Current(account, round)
	if err != nil {
		return Record{}, err
	}
	if exists && before.Frozen == frozen {
		return before, nil
	}
	after, err := s.next(account, frozen, round, before)
	if err != nil {
		return Record{}, err
	}
	return s.persist(after, before, exists)
}

func (s *Store) next(account [32]byte, frozen bool, round uint64, before Record) (Record, error) {
	domain, err := s.accountDomain(account)
	if err != nil {
		return Record{}, err
	}
	if before.Revision == ^uint64(0) {
		return Record{}, ErrRevision
	}
	if before.Updated > round {
		return Record{}, ErrRound
	}
	return Record{Domain: domain, Revision: before.Revision + 1, Updated: round, Frozen: frozen}, nil
}

func (s *Store) persist(after, before Record, exists bool) (Record, error) {
	records, err := s.prepare(after, before, exists)
	if err != nil {
		return Record{}, err
	}
	if err = s.kv.Apply(records); err != nil {
		return Record{}, err
	}
	return after, nil
}
