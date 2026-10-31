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

func (cs *roundCowState) assetPrivacyCleanupRecord(body lifecyclebody.Body, sender basics.Address) (retirement.Record, error) {
	record, exists, err := cs.readAssetPrivacyRetirement(basics.AssetIndex(body.Asset))
	if err != nil {
		return retirement.Record{}, err
	}
	if !exists {
		return retirement.Record{}, retirement.ErrActive
	}
	if record.Manager != sender || record.Policy != body.Policy || record.Suite != body.Suite {
		return retirement.Record{}, retirement.ErrRecord
	}
	if record.Epoch != body.ExpectedEpoch {
		return retirement.Record{}, retirement.ErrEpoch
	}
	return record, cs.assetPrivacyRetirementAuthority(basics.AssetIndex(body.Asset), sender)
}

func (cs *roundCowState) checkAssetPrivacyLifecycleState(body lifecyclebody.Body) error {
	if body.Operation != lifecyclebody.Cleanup {
		if err := cs.checkAssetPrivacyActive(basics.AssetIndex(body.Asset)); err != nil {
			return err
		}
	}
	if body.Operation == lifecyclebody.Adopt {
		return nil
	}
	return cs.checkAssetPrivacyLifecyclePolicy(body)
}
