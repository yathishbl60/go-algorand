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
	"github.com/algorand/go-algorand/internal/assetprivacy/retirement"
)

func (cs *roundCowState) finalizeAssetPrivacyDestruction(asset basics.AssetIndex, sender basics.Address, record retirement.Record) error {
	if err := cs.checkAssetPrivacyDestructionContext(asset, sender, record); err != nil {
		return err
	}
	protected, err := cs.assetPrivacyFinalKeys(asset)
	if err != nil {
		return err
	}
	for _, key := range protected {
		if err = cs.kvDel(key); err != nil {
			return err
		}
	}
	return cs.removeAssetPrivacyInventoryMetadata(asset)
}

func (cs *roundCowState) assetPrivacyFinalKeys(asset basics.AssetIndex) ([3]string, error) {
	protected, err := assetPrivacyProtectedKeys(asset)
	if err != nil {
		return protected, err
	}
	remaining, err := inventory.Keys(assetPrivacyInventoryKV{cs}, uint64(asset), len(protected)+1)
	if err != nil {
		return protected, err
	}
	for _, key := range remaining {
		if key != protected[0] && key != protected[1] && key != protected[2] {
			return protected, errAssetPrivacyCleanup
		}
	}
	return protected, nil
}
