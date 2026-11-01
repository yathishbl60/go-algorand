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

package query

import (
	"github.com/algorand/go-algorand/internal/assetprivacy/outputs"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

type OutputPage struct {
	Round, Asset uint64
	PolicyID     [64]byte
	Root         tree.Hash
	outputs.Page
}

type NoteWitness struct {
	Round, Asset uint64
	PolicyID     [64]byte
	Witness      tree.Witness
}

func CheckOutputState(snapshot Snapshot) error { return CheckLiveState(snapshot) }

func CheckLiveState(snapshot Snapshot) error {
	if snapshot.Retired != nil || snapshot.Closure != nil {
		return ErrUnavailable
	}
	if snapshot.Policy == nil {
		return policy.ErrMissing
	}
	return nil
}

// Page caps apply before any tracker reads or output allocation.
func CheckPage(start uint64, limit uint32) error {
	if limit == 0 || limit > outputs.MaxPageSize || start > tree.Capacity {
		return outputs.ErrPage
	}
	return nil
}
