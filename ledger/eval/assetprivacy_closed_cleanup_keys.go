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

package eval

import (
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/internal/assetprivacy/closure"
	"github.com/algorand/go-algorand/internal/assetprivacy/inventory"
)

func (cs *roundCowState) assetPrivacyClosedCleanupKeys(asset basics.AssetIndex, maximum uint16) ([]string, error) {
	key, err := closure.Key(uint64(asset))
	if err != nil {
		return nil, err
	}
	keys, err := inventory.Keys(assetPrivacyInventoryKV{cs}, uint64(asset), int(maximum)+1)
	if err != nil {
		return nil, err
	}
	selected := make([]string, 0, maximum)
	for _, candidate := range keys {
		if candidate != key && len(selected) < int(maximum) {
			selected = append(selected, candidate)
		}
	}
	return selected, nil
}

func (cs *roundCowState) assetPrivacyClosedCleanupLast(asset basics.AssetIndex) (bool, error) {
	keys, err := inventory.Keys(assetPrivacyInventoryKV{cs}, uint64(asset), 2)
	if err != nil {
		return false, err
	}
	if len(keys) == 0 {
		return false, inventory.ErrIndex
	}
	if len(keys) != 1 {
		return false, nil
	}
	marker, _ := closure.Key(uint64(asset))
	if keys[0] != marker {
		return false, inventory.ErrIndex
	}
	return true, nil
}
