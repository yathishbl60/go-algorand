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
	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
)

// assetPrivacyKeysView exposes reads without the registry-bypassing Put method.
type assetPrivacyKeysView struct{ store *keys.Store }

// OpenAssetPrivacyKeys reads current/history records without granting write access.
// Use PutAssetPrivacyKey for atomic history, pointer and authorization-root updates.
func (cs *roundCowState) OpenAssetPrivacyKeys(asset basics.AssetIndex, account basics.Address, role keys.Role) (*assetPrivacyKeysView, error) {
	store, err := cs.openAssetPrivacyKeys(asset, account, role)
	if err != nil {
		return nil, err
	}
	return &assetPrivacyKeysView{store: store}, nil
}

func (view *assetPrivacyKeysView) Current(round uint64) (keys.Record, bool, error) {
	return view.store.Current(round)
}

func (view *assetPrivacyKeysView) Epoch(epoch, round uint64) (keys.Record, bool, error) {
	return view.store.Epoch(epoch, round)
}
