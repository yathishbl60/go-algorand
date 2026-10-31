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
	"errors"
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/internal/assetprivacy/accounting"
	"github.com/algorand/go-algorand/internal/assetprivacy/inventory"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/retirement"
)

var errAssetPrivacyCleanupReady = errors.New("native privacy cleanup is complete; use authorized asset destruction")

func assetPrivacyProtectedKeys(asset basics.AssetIndex) ([3]string, error) {
	if asset == 0 {
		return [3]string{}, retirement.ErrRecord
	}
	declaration, _ := policy.RecordKey(uint64(asset))
	supply, _ := accounting.SupplyKey(uint64(asset))
	marker, _ := retirement.Key(uint64(asset))
	return [3]string{declaration, supply, marker}, nil
}

func (cs *roundCowState) assetPrivacyCleanupKeys(asset basics.AssetIndex, maximum uint16) ([]string, error) {
	protected, err := assetPrivacyProtectedKeys(asset)
	if err != nil {
		return nil, err
	}
	keys, err := inventory.Keys(assetPrivacyInventoryKV{cs}, uint64(asset), int(maximum)+len(protected))
	if err != nil {
		return nil, err
	}
	return selectAssetPrivacyCleanupKeys(keys, protected, int(maximum))
}

func selectAssetPrivacyCleanupKeys(keys []string, protected [3]string, maximum int) ([]string, error) {
	var selected []string
	for _, key := range keys {
		if key == protected[0] || key == protected[1] || key == protected[2] {
			continue
		}
		selected = append(selected, key)
		if len(selected) == maximum {
			break
		}
	}
	if len(selected) == 0 {
		return nil, errAssetPrivacyCleanupReady
	}
	return selected, nil
}
