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
	"github.com/algorand/go-algorand/internal/assetprivacy/spendplan"
	"github.com/algorand/go-algorand/internal/assetprivacy/statement"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

func (cs *roundCowState) assetPrivacySpendConfiguration(asset basics.AssetIndex) (spendplan.Config, [64]byte, error) {
	declaration, exists, err := cs.GetAssetPrivacyPolicy(asset)
	if err != nil {
		return spendplan.Config{}, [64]byte{}, err
	}
	if !exists {
		return spendplan.Config{}, [64]byte{}, policy.ErrMissing
	}
	identity, err := declaration.Identity(cs.GenesisHash(), uint64(asset))
	if err != nil {
		return spendplan.Config{}, [64]byte{}, err
	}
	config := spendplan.Config{Domain: tree.Domain{Genesis: cs.GenesisHash(), Asset: uint64(asset), Policy: identity}}
	config.EligibilityRoot, _, err = cs.AssetPrivacyEligibilityRoot(asset)
	return config, declaration.Suite, err
}

func (cs *roundCowState) assetPrivacySpendContext(asset basics.AssetIndex, bodies AssetPrivacySpendBodies) (assetPrivacySpendContext, error) {
	config, suite, err := cs.assetPrivacySpendConfiguration(asset)
	if err != nil {
		return assetPrivacySpendContext{}, err
	}
	if bodies.Limits.MaxGroupSize != cs.proto.MaxTxGroupSize {
		return assetPrivacySpendContext{}, statement.ErrLimit
	}
	group, err := statement.Bind(cs.GenesisHash(), suite, bodies.Target, bodies.Group, bodies.Limits)
	if err != nil {
		return assetPrivacySpendContext{}, err
	}
	return assetPrivacySpendContext{config: config, group: group,
		transaction: [32]byte(bodies.Group[bodies.Target].ID()), round: uint64(cs.Round())}, nil
}
