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

func (cs *roundCowState) checkAssetPrivacyCleanupPolicy(body lifecyclebody.Body) error {
	exists, err := cs.assetPrivacyExists(basics.AssetIndex(body.Asset))
	if err != nil {
		return err
	}
	if exists {
		return cs.checkAssetPrivacyLifecyclePolicy(body)
	}
	record, found, err := cs.readAssetPrivacyClosure(basics.AssetIndex(body.Asset))
	if err != nil {
		return err
	}
	if !found || record.Policy != body.Policy || record.Suite != body.Suite {
		return closure.ErrRecord
	}
	return nil
}

func (cs *roundCowState) assetPrivacyClosedCleanupRecord(body lifecyclebody.Body, sender basics.Address) (closure.Record, error) {
	record, exists, err := cs.readAssetPrivacyClosure(basics.AssetIndex(body.Asset))
	if err != nil {
		return closure.Record{}, err
	}
	if !exists || record.Manager != sender || record.Policy != body.Policy || record.Suite != body.Suite {
		return closure.Record{}, closure.ErrRecord
	}
	if record.Epoch != body.ExpectedEpoch || record.Epoch == ^uint64(0) {
		return closure.Record{}, closure.ErrRecord
	}
	return record, cs.checkAssetPrivacyClosedFixedRecords(basics.AssetIndex(body.Asset))
}

func (cs *roundCowState) checkAssetPrivacyClosedFixedRecords(asset basics.AssetIndex) error {
	keys, err := assetPrivacyProtectedKeys(asset)
	if err != nil {
		return err
	}
	for _, key := range keys {
		_, exists, err := cs.kvGet(key)
		if err != nil {
			return err
		}
		if exists {
			return closure.ErrRecord
		}
	}
	return nil
}
