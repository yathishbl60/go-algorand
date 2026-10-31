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
)

func (cs *roundCowState) removeAssetPrivacyClosingRecords(asset basics.AssetIndex) error {
	protected, err := assetPrivacyProtectedKeys(asset)
	if err != nil {
		return err
	}
	for _, key := range protected {
		if err = cs.kvDel(key); err != nil {
			return err
		}
	}
	return nil
}

func (cs *roundCowState) checkAssetPrivacyClosureParameters(asset basics.AssetIndex, record closure.Record) error {
	params, creator, err := cs.AssetParams(asset)
	if err != nil {
		return err
	}
	if creator != record.Creator || params.Total != record.Total || params.Manager != record.Manager {
		return closure.ErrRecord
	}
	return nil
}

// Only the authenticated wrapper can first remove policy, supply and retirement.
func (cs *roundCowState) checkAssetPrivacyClosingGuard(asset basics.AssetIndex) error {
	if cs.nativePrivacyDestructionAsset != asset {
		return errAssetPrivacyCleanup
	}
	record, exists, err := cs.readAssetPrivacyClosure(asset)
	if err != nil {
		return err
	}
	if !exists || record.Created != uint64(cs.Round()) || record.Epoch != 1 {
		return errAssetPrivacyCleanup
	}
	return cs.checkAssetPrivacyClosureParameters(asset, record)
}
