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

func (cs *roundCowState) finishClosedAssetPrivacyCleanup(asset basics.AssetIndex, payer basics.Address) error {
	key, err := closure.Key(uint64(asset))
	if err != nil {
		return err
	}
	if err = cs.kvDel(key); err != nil {
		return err
	}
	return cs.removeClosedAssetPrivacyInventory(asset, payer)
}

func (cs *roundCowState) removeClosedAssetPrivacyInventory(asset basics.AssetIndex, payer basics.Address) error {
	record, exists, err := inventory.Read(assetPrivacyInventoryKV{cs}, uint64(asset))
	if err != nil {
		return err
	}
	if !exists || record.Items != 0 || record.Bytes != 0 {
		return inventory.ErrIndex
	}
	key, err := inventory.MetadataKey(uint64(asset))
	if err != nil {
		return err
	}
	return cs.deleteAssetPrivacyInventoryMetadata(payer, key)
}
