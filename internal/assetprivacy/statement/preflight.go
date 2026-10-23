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

package statement

import "github.com/algorand/go-algorand/data/transactions"

func preflight(genesis [32]byte, target uint32, group []transactions.Transaction, limits Limits) error {
	if err := limits.validate(); err != nil {
		return err
	}
	if len(group) == 0 || len(group) > limits.MaxGroupSize || uint64(target) >= uint64(len(group)) {
		return ErrGroup
	}
	remaining := limits.MaxGroupEstimate
	for i := range group {
		if group[i].GenesisHash != genesis || group[i].FirstValid > group[i].LastValid {
			return ErrGroup
		}
		if err := reserveBody(&group[i], limits.MaxBodyEstimate, &remaining); err != nil {
			return err
		}
	}
	return nil
}

func reserveBody(tx *transactions.Transaction, maximum uint64, remaining *uint64) error {
	estimate := tx.Msgsize()
	if estimate <= 0 || uint64(estimate) > maximum || uint64(estimate) > *remaining {
		return ErrLimit
	}
	*remaining -= uint64(estimate)
	return nil
}
