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
	"github.com/algorand/go-algorand/internal/assetprivacy/lifecyclebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/retirement"
)

func (cs *roundCowState) cleanupAssetPrivacy(body lifecyclebody.Body, sender basics.Address) error {
	asset := basics.AssetIndex(body.Asset)
	record, err := cs.assetPrivacyCleanupRecord(body, sender)
	if err != nil {
		return err
	}
	if err = cs.checkAssetPrivacyRetirementSupply(asset); err != nil {
		return err
	}
	if err = cs.reserveAssetPrivacyProofChecks(1); err != nil {
		return err
	}
	return cs.applyAssetPrivacyCleanup(asset, body.CleanupLimit, record)
}

func (cs *roundCowState) applyAssetPrivacyCleanup(asset basics.AssetIndex, maximum uint16, record retirement.Record) error {
	keys, err := cs.assetPrivacyCleanupKeys(asset, maximum)
	if err != nil {
		return err
	}
	for _, key := range keys {
		if err = cs.kvDel(key); err != nil {
			return err
		}
	}
	record.Epoch++
	return cs.writeAssetPrivacyRetirement(asset, record)
}
