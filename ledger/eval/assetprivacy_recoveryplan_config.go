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
	"github.com/algorand/go-algorand/internal/assetprivacy/recoveryplan"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

func (cs *roundCowState) assetPrivacyRecoveryConfiguration(asset basics.AssetIndex) (recoveryplan.Config, [64]byte, error) {
	declaration, err := cs.assetPrivacyRecoveryDeclaration(asset)
	if err != nil {
		return recoveryplan.Config{}, [64]byte{}, err
	}
	identity, err := declaration.Identity(cs.GenesisHash(), uint64(asset))
	if err != nil {
		return recoveryplan.Config{}, [64]byte{}, err
	}
	authority, err := cs.assetPrivacyCurrentClawback(asset)
	if err != nil {
		return recoveryplan.Config{}, [64]byte{}, err
	}
	return recoveryplan.Config{Domain: tree.Domain{Genesis: cs.GenesisHash(), Asset: uint64(asset), Policy: identity},
		Authority: authority}, declaration.Suite, nil
}

func (cs *roundCowState) assetPrivacyRecoveryContext(asset basics.AssetIndex, bodies AssetPrivacySpendBodies) (assetPrivacyRecoveryContext, error) {
	config, suite, err := cs.assetPrivacyRecoveryConfiguration(asset)
	if err != nil {
		return assetPrivacyRecoveryContext{}, err
	}
	body, err := cs.assetPrivacyBindBodies(suite, bodies)
	if err != nil {
		return assetPrivacyRecoveryContext{}, err
	}
	if [32]byte(body.sender) != config.Authority {
		return assetPrivacyRecoveryContext{}, recoveryplan.ErrAuthority
	}
	return assetPrivacyRecoveryContext{config: config, assetPrivacyBodyContext: body}, nil
}
