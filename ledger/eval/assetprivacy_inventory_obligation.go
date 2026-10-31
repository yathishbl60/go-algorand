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
	"github.com/algorand/go-algorand/internal/assetprivacy/storage"
	"github.com/algorand/go-algorand/ledger/ledgercore"
)

func (cs *roundCowState) assetPrivacyInventoryObligation(obligation storage.Obligation, writes []inventory.Write) (storage.Obligation, error) {
	for _, write := range writes {
		before, exists, err := cs.kvGet(write.Key)
		if err != nil {
			return storage.Obligation{}, err
		}
		obligation, err = obligation.Change(uint64(len(write.Key)), uint64(len(before)), exists, uint64(len(write.Value)), write.Value != nil)
		if err != nil {
			return storage.Obligation{}, err
		}
	}
	return obligation, nil
}

func (cs *roundCowState) commitAssetPrivacyInventory(payer basics.Address, obligation storage.Obligation, writes []inventory.Write) error {
	if err := cs.storeAssetPrivacyObligation(payer, obligation); err != nil {
		return err
	}
	for _, write := range writes {
		cs.mods.AddKvMod(write.Key, ledgercore.KvValueDelta{Data: append([]byte(nil), write.Value...)})
	}
	return nil
}
