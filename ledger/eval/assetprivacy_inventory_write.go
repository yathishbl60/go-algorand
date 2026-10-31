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
	"strings"
)

type assetPrivacyInventoryKV struct{ state *roundCowState }

func (kv assetPrivacyInventoryKV) Get(key string) ([]byte, bool, error) {
	return kv.state.kvGet(key)
}

func (cs *roundCowState) chargeAssetPrivacyInventory(asset uint64, payer basics.Address, key string, before []byte, exists bool, after []byte, present bool, obligation storage.Obligation) error {
	change := inventory.Change{Key: key, Before: uint64(len(before)), After: uint64(len(after)),
		Existed: exists, Present: present, Initialize: !exists && present && strings.HasPrefix(key, "aprv:p:")}
	writes, err := inventory.Plan(assetPrivacyInventoryKV{cs}, asset, change)
	if err != nil {
		return err
	}
	next, err := cs.assetPrivacyInventoryObligation(obligation, writes)
	if err != nil {
		return err
	}
	return cs.commitAssetPrivacyInventory(payer, next, writes)
}
