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
	"github.com/algorand/go-algorand/internal/assetprivacy/disclosure"
)

// SetAssetPrivacyDisclosureTargets persists a preauthorized-manager assignment.
// Caller must still verify PQ signatures, key possession, suite, fees and funding.
func (cs *roundCowState) SetAssetPrivacyDisclosureTargets(asset basics.AssetIndex, manager, auditor basics.Address, expectedEpoch uint64) (disclosure.Record, error) {
	params, err := cs.assetPrivacyDisclosureAuthority(asset, manager)
	if err != nil {
		return disclosure.Record{}, err
	}
	store, err := cs.openAssetPrivacyDisclosureTargets(asset)
	if err != nil {
		return disclosure.Record{}, err
	}
	return store.Set(auditor, params.Clawback, expectedEpoch, uint64(cs.Round()))
}
