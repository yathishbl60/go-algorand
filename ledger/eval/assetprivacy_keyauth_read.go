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

// AssetPrivacyKeyAuthorizationRoot never accepts historical note roots.
// Verification must recheck this current root after preceding group members.
func (cs *roundCowState) AssetPrivacyKeyAuthorizationRoot(asset basics.AssetIndex) (keyauth.Root, uint64, error) {
	store, err := cs.OpenAssetPrivacyKeyAuthorization(asset)
	if err != nil {
		return keyauth.Root{}, 0, err
	}
	return store.Snapshot()
}

// AssetPrivacyKeyAuthorizationWitness is an internal local proving interface.
// It does not authorize a transaction or define a public account-query endpoint.
func (cs *roundCowState) AssetPrivacyKeyAuthorizationWitness(record keys.Record) (tree.Witness, error) {
	asset := basics.AssetIndex(record.Domain.Asset)
	if err := cs.checkCurrentAssetPrivacyKey(record); err != nil {
		return tree.Witness{}, err
	}
	registry, err := cs.OpenAssetPrivacyKeyAuthorization(asset)
	if err != nil {
		return tree.Witness{}, err
	}
	return registry.Witness(record, uint64(cs.Round()))
}
