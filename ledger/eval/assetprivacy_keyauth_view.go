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
	"github.com/algorand/go-algorand/internal/assetprivacy/keyauth"
	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

// assetPrivacyKeyAuthorizationView cannot update the root independently of keys.
type assetPrivacyKeyAuthorizationView struct{ store *keyauth.Store }

// OpenAssetPrivacyKeyAuthorization exposes current snapshot/witness reads only.
func (cs *roundCowState) OpenAssetPrivacyKeyAuthorization(asset basics.AssetIndex) (*assetPrivacyKeyAuthorizationView, error) {
	store, err := cs.openAssetPrivacyKeyAuthorization(asset)
	if err != nil {
		return nil, err
	}
	return &assetPrivacyKeyAuthorizationView{store: store}, nil
}

func (view *assetPrivacyKeyAuthorizationView) Snapshot() (keyauth.Root, uint64, error) {
	return view.store.Snapshot()
}

func (view *assetPrivacyKeyAuthorizationView) Witness(record keys.Record, round uint64) (tree.Witness, error) {
	return view.store.Witness(record, round)
}
