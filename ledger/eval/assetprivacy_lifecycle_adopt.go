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
	"errors"

	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/internal/assetprivacy/lifecyclebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

var errAssetPrivacyAdoptionState = errors.New("native privacy adoption requires absent private supply state")

// Adoption creates no private value or keys and preserves all public holdings.
func (cs *roundCowState) adoptAssetPrivacyLifecycle(body lifecyclebody.Body, manager basics.Address) error {
	asset := basics.AssetIndex(body.Asset)
	if err := cs.checkAssetPrivacyAdoptionSupply(asset); err != nil {
		return err
	}
	declaration := policy.Policy{Modes: body.Modes, Capabilities: body.Capabilities, Disclosures: body.Disclosures}
	record := policy.Record{Policy: declaration, InitialCapabilities: body.Capabilities,
		Suite: body.Suite, Created: uint64(cs.Round())}
	if err := cs.AdoptAssetPrivacyPolicy(asset, manager, record); err != nil {
		return err
	}
	_, err := cs.SetAssetPrivacyDisclosureTargets(asset, manager, basics.Address(body.Auditor), 0)
	return err
}

func (cs *roundCowState) checkAssetPrivacyAdoptionSupply(asset basics.AssetIndex) error {
	_, exists, err := cs.GetAssetPrivacySupply(asset)
	if err != nil {
		return err
	}
	if exists {
		return errAssetPrivacyAdoptionState
	}
	return nil
}
