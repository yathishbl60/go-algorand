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
	"github.com/algorand/go-algorand/internal/assetprivacy/inventory"
	"github.com/algorand/go-algorand/ledger/ledgercore"
)

func (cs *roundCowState) removeAssetPrivacyInventoryMetadata(asset basics.AssetIndex) error {
	record, exists, err := inventory.Read(assetPrivacyInventoryKV{cs}, uint64(asset))
	if err != nil {
		return err
	}
	if !exists || record.Items != 0 || record.Bytes != 0 {
		return errAssetPrivacyCleanup
	}
	payer, err := cs.assetPrivacyStorageOwner(uint64(asset))
	if err != nil {
		return err
	}
	key, _ := inventory.MetadataKey(uint64(asset))
	return cs.deleteAssetPrivacyInventoryMetadata(payer, key)
}

func (cs *roundCowState) deleteAssetPrivacyInventoryMetadata(payer basics.Address, key string) error {
	before, exists, err := cs.kvGet(key)
	if err != nil {
		return err
	}
	if !exists {
		return inventory.ErrMissing
	}
	next, err := cs.assetPrivacyNextObligation(payer, key, before, true, nil, false)
	if err != nil {
		return err
	}
	if err = cs.storeAssetPrivacyObligation(payer, next); err != nil {
		return err
	}
	cs.mods.AddKvMod(key, ledgercore.KvValueDelta{})
	return cs.releaseAssetPrivacyEmptyCounter(payer, next)
}
