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
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/proofplan"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

func (cs *roundCowState) assetPrivacyOutputProofConfig(asset basics.AssetIndex) (proofplan.Config, error) {
	declaration, exists, err := cs.GetAssetPrivacyPolicy(asset)
	if err != nil {
		return proofplan.Config{}, err
	}
	if !exists {
		return proofplan.Config{}, policy.ErrMissing
	}
	identity, err := declaration.Identity(cs.GenesisHash(), uint64(asset))
	if err != nil {
		return proofplan.Config{}, err
	}
	config := proofplan.Config{Domain: tree.Domain{Genesis: cs.GenesisHash(), Asset: uint64(asset), Policy: identity},
		Disclosures: declaration.Policy.Disclosures}
	return cs.assetPrivacyOutputProofState(asset, config)
}

func (cs *roundCowState) assetPrivacyOutputProofState(asset basics.AssetIndex, config proofplan.Config) (proofplan.Config, error) {
	targets, _, err := cs.GetAssetPrivacyDisclosureTargets(asset)
	if err != nil {
		return proofplan.Config{}, err
	}
	config.Auditor, config.Controller = targets.Auditor, targets.Controller
	config.KeyRoot, _, err = cs.AssetPrivacyKeyAuthorizationRoot(asset)
	if err != nil {
		return proofplan.Config{}, err
	}
	config.EligibilityRoot, _, err = cs.AssetPrivacyEligibilityRoot(asset)
	return config, err
}
