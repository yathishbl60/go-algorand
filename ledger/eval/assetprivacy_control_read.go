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
	"github.com/algorand/go-algorand/internal/assetprivacy/control"
)

// GetAssetPrivacyControl returns the marker independently of public holding state.
func (cs *roundCowState) GetAssetPrivacyControl(asset basics.AssetIndex, account basics.Address) (control.Record, bool, error) {
	store, err := cs.OpenAssetPrivacyControl(asset, account)
	if err != nil {
		return control.Record{}, false, err
	}
	return store.Current(uint64(cs.Round()))
}

// AssetPrivacyFrozen reports current effective eligibility, never a proof result.
func (cs *roundCowState) AssetPrivacyFrozen(asset basics.AssetIndex, account basics.Address) (bool, uint64, error) {
	marker, exists, err := cs.GetAssetPrivacyControl(asset, account)
	if err != nil {
		return false, 0, err
	}
	params, _, err := cs.AssetParams(asset)
	if err != nil {
		return false, 0, err
	}
	holding, hasHolding, err := cs.GetAssetHolding(account, asset)
	if err != nil {
		return false, 0, err
	}
	return control.EffectiveFrozen(marker, exists, hasHolding, holding.Frozen, params.DefaultFrozen), marker.Epoch, nil
}
