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

// PutAssetPrivacyKey is internal persistence, not registration authorization.
// Callers must prove account/issuer authority, PQ/KEM possession, suite and fees.
// RecipientSpend stores a note owner commitment, whose secret needs a ZK proof.
func (cs *roundCowState) PutAssetPrivacyKey(record keys.Record) error {
	asset := basics.AssetIndex(record.Domain.Asset)
	store, err := cs.OpenAssetPrivacyKeys(asset, basics.Address(record.Domain.Account), record.Domain.Role)
	if err != nil {
		return err
	}
	if err = cs.checkAssetPrivacyKeyRole(record); err != nil {
		return err
	}
	return store.Put(record, uint64(cs.Round()))
}
