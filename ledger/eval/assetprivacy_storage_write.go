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
	"github.com/algorand/go-algorand/internal/assetprivacy/storage"
	"github.com/algorand/go-algorand/ledger/ledgercore"
)

// Every native KV write reaches this before its actual data delta is changed.
func (cs *roundCowState) chargeAssetPrivacyStorage(key string, value []byte, present bool) error {
	if !cs.proto.EnableNativeAssetPrivacy {
		return nil
	}
	asset, native, err := storage.Asset(key)
	if err != nil || !native {
		return err
	}
	if err = cs.checkAssetPrivacyRetiredWrite(asset, key, present); err != nil {
		return err
	}
	return cs.chargeAssetPrivacyKV(asset, key, value, present)
}

func (cs *roundCowState) chargeAssetPrivacyKV(asset uint64, key string, value []byte, present bool) error {
	before, exists, err := cs.kvGet(key)
	if err != nil || (!exists && !present) {
		return err
	}
	if present && value == nil {
		return storage.ErrEncoding
	}
	payer, err := cs.assetPrivacyStorageOwner(asset)
	if err != nil {
		return err
	}
	obligation, err := cs.assetPrivacyNextObligation(payer, key, before, exists, value, present)
	if err != nil {
		return err
	}
	return cs.chargeAssetPrivacyInventory(asset, payer, key, before, exists, value, present, obligation)
}

func (cs *roundCowState) storeAssetPrivacyObligation(payer basics.Address, obligation storage.Obligation) error {
	data, err := cs.assetPrivacyFundingAccount(payer, obligation)
	if err != nil {
		return err
	}
	encoded, err := storage.Encode(obligation)
	if err != nil {
		return err
	}
	key, err := storage.Key(payer)
	if err != nil {
		return err
	}
	cs.mods.AddKvMod(key, ledgercore.KvValueDelta{Data: encoded})
	return cs.Put(payer, data) // Include creator in subsequent withdrawal/close checks.
}
