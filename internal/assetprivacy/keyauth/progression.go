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

package keyauth

import "github.com/algorand/go-algorand/internal/assetprivacy/keys"

func (s *Store) checkUpdate(after, before keys.Record, exists bool, round uint64) error {
	if err := s.checkDomain(after.Domain); err != nil {
		return err
	}
	if after.Created != round {
		return ErrRecord
	}
	if !exists {
		if !emptyRecord(before) || after.Epoch != 1 {
			return ErrHistory
		}
		return nil
	}
	return checkPrevious(after, before, round)
}

func checkPrevious(after, before keys.Record, round uint64) error {
	if err := before.Validate(); err != nil {
		return err
	}
	if before.Domain != after.Domain || before.Created > round {
		return ErrHistory
	}
	if before.Epoch == ^uint64(0) || after.Epoch != before.Epoch+1 || after.Created < before.Created {
		return ErrHistory
	}
	return nil
}

func emptyRecord(record keys.Record) bool {
	return record.Domain == (keys.Domain{}) && record.Epoch == 0 && record.Created == 0 && len(record.PublicKey) == 0
}
