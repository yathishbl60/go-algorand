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
	"github.com/algorand/go-algorand/internal/assetprivacy/lifecyclebody"
)

func (cs *roundCowState) cleanupClosedAssetPrivacy(body lifecyclebody.Body, sender basics.Address) error {
	record, err := cs.assetPrivacyClosedCleanupRecord(body, sender)
	if err != nil {
		return err
	}
	if err = cs.reserveAssetPrivacyProofChecks(1); err != nil {
		return err
	}
	asset := basics.AssetIndex(body.Asset)
	keys, err := cs.assetPrivacyClosedCleanupKeys(asset, body.CleanupLimit)
	if err != nil {
		return err
	}
	return cs.applyClosedAssetPrivacyCleanup(asset, keys, record)
}

func (cs *roundCowState) applyClosedAssetPrivacyCleanup(asset basics.AssetIndex, keys []string, record closure.Record) error {
	for _, key := range keys {
		if err := cs.kvDel(key); err != nil {
			return err
		}
	}
	last, err := cs.assetPrivacyClosedCleanupLast(asset)
	if err != nil {
		return err
	}
	if last {
		return cs.finishClosedAssetPrivacyCleanup(asset, basics.Address(record.Creator))
	}
	record.Epoch++
	return cs.writeAssetPrivacyClosure(asset, record)
}
